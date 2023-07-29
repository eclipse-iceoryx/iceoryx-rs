// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: © Contributors to the iceoryx-rs project
// SPDX-FileContributor: Mathias Kraus

use crate::marker::ShmSend;
use crate::{RawSample, SubscribeState};

use std::marker::PhantomData;
use std::time::{Duration, SystemTime};

use std::mem::ManuallyDrop;
use std::ops::Deref;

//TODO impl debug for Sample with T: Debug
/// An immutable sample shared between multiple subscriber
pub struct Sample<T: ?Sized, S: ffi::SubscriberStrongRef> {
    data: RawSample<T>,
    ffi_sub: ManuallyDrop<S>,
}

impl<T: ?Sized, S: ffi::SubscriberStrongRef> Deref for Sample<T, S> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: `as_payload_ptr` returns a non-null ptr
        unsafe { &*self.data.as_payload_ptr() }
    }
}

impl<T: ?Sized, S: ffi::SubscriberStrongRef> Drop for Sample<T, S> {
    fn drop(&mut self) {
        self.ffi_sub.as_ref().release(self.data);
        unsafe {
            ManuallyDrop::<S>::drop(&mut self.ffi_sub);
        }
    }
}

impl<T: ?Sized, S: ffi::SubscriberStrongRef> Sample<T, S> {
    fn into_raw_parts(mut self) -> (RawSample<T>, S) {
        let sample = self.data;
        let ffi_sub = ManuallyDrop::into_inner(self.ffi_sub.clone());
        unsafe {
            ManuallyDrop::<S>::drop(&mut self.ffi_sub);
        }
        std::mem::forget(self); // forget `self` to not call drop
        (sample, ffi_sub)
    }

    /// Convert into `RawSample`
    ///
    /// The sample must manually be managed by calling either `release_raw` via the corresponding
    /// subscriber. Not doing either will lead to a memory leak.
    /// `RawSample` is a thin wrapper around a raw pointer which is guaranteed to be non-null
    /// but it might be dangling if `release_raw` was already called on the specific raw sample.
    pub fn into_raw(self) -> RawSample<T> {
        let (sample, _) = self.into_raw_parts();
        sample
    }
}

impl<S: ffi::SubscriberStrongRef> Sample<[u8], S> {
    /// Get a reference to a T
    ///
    /// If the size and alignment of T do not match with the alignment of the underlying buffer, None is returned.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the [u8] is actually a T. It is undefined behavior if the underlying data is not a T.
    pub unsafe fn try_as<T: ShmSend>(&self) -> Option<&T> {
        let chunk_header = self.data.chunk_header();
        let payload_size = chunk_header.get_user_payload_size() as usize;
        let payload_alignment = chunk_header.get_user_payload_alignment() as usize;

        if payload_size >= std::mem::size_of::<T>()
            && payload_alignment >= std::mem::align_of::<T>()
        {
            let payload = self.data.cast::<u8>().as_payload_ptr();
            Some(&*(std::mem::transmute::<*const u8, *const T>(payload)))
        } else {
            None
        }
    }
}

/// Access to the sample receiver queue of the subscriber
pub struct SampleReceiver<T: ?Sized, S: ffi::SubscriberStrongRef> {
    ffi_sub: S,
    condition_variable: Box<ffi::ConditionVariable>,
    phantom: PhantomData<T>,
}

impl<T: ?Sized, S: ffi::SubscriberStrongRef> SampleReceiver<T, S> {
    pub(super) fn new(ffi_sub: S) -> Self {
        let condition_variable = ffi::ConditionVariable::new();
        ffi_sub.as_ref().set_condition_variable(&condition_variable);

        SampleReceiver {
            ffi_sub,
            condition_variable,
            phantom: PhantomData,
        }
    }

    /// The current subscription state of the corresponding subscriber
    ///
    /// After `SubscriberBuilder::create` this will immediately be [`SubscribeState::Subscribed`] but after
    /// `InactiveSubscriber::subscribe` it might take up to 50 milliseconds until `RouDi` runs its
    /// discovery loop and the subscriber will be subscribed to the publisher.
    pub fn subscription_state(&self) -> SubscribeState {
        self.ffi_sub.as_ref().subscription_state()
    }

    /// Blocking wait for samples
    ///
    /// This method unblock when either new samples are available, the timeout duration elapsed or the
    /// `SampleReceiver` was stopped.
    pub fn wait_for_samples(&self, timeout: Duration) -> SampleReceiverWaitState {
        if !self.ffi_sub.as_ref().is_condition_variable_set() {
            return SampleReceiverWaitState::Stopped;
        }
        if self.has_data() {
            return SampleReceiverWaitState::SamplesAvailable;
        }

        let entry_time = SystemTime::now();
        while let Some(remaining_timeout) = {
            let elapsed = entry_time.elapsed().unwrap_or(timeout);
            timeout.checked_sub(elapsed)
        } {
            self.condition_variable.timed_wait(remaining_timeout);
            if self.has_data() {
                return SampleReceiverWaitState::SamplesAvailable;
            }
        }

        if self.ffi_sub.as_ref().is_condition_variable_set() {
            SampleReceiverWaitState::Timeout
        } else {
            SampleReceiverWaitState::Stopped
        }
    }

    /// Checks it there are samples in the receiver queue
    pub fn has_data(&self) -> bool {
        self.ffi_sub.as_ref().has_chunks()
    }

    /// Clears the receiver queue and release all the samples from the queue
    pub fn clear(&self) {
        self.ffi_sub.as_ref().clear();
    }

    /// Releases a raw sample which will not be used anymore
    pub fn release_raw(&self, sample: RawSample<T>) {
        self.ffi_sub.as_ref().release(sample);
    }
}

impl<T, S: ffi::SubscriberStrongRef> SampleReceiver<T, S> {
    /// Takes a sample from the receiver queue
    ///
    /// If the receiver queue is empty, `None` will be returned.
    pub fn take(&self) -> Option<Sample<T, S>> {
        self.ffi_sub
            .as_ref()
            .try_take::<T>()
            .map(|data| Sample::<T, S> {
                data,
                ffi_sub: ManuallyDrop::new(self.ffi_sub.clone()),
            })
    }
}

impl<T, S: ffi::SubscriberStrongRef> SampleReceiver<[T], S> {
    /// Takes a sample from the receiver queue
    ///
    /// If the receiver queue is empty, `None` will be returned.
    pub fn take(&self) -> Option<Sample<[T], S>> {
        self.ffi_sub
            .as_ref()
            .try_take_slice::<T>()
            .map(|data| Sample::<[T], S> {
                data,
                ffi_sub: ManuallyDrop::new(self.ffi_sub.clone()),
            })
    }
}

impl<T: ?Sized, S: ffi::SubscriberStrongRef> Drop for SampleReceiver<T, S> {
    fn drop(&mut self) {
        self.ffi_sub.as_ref().unset_condition_variable();
        self.ffi_sub.as_ref().unsubscribe();
    }
}

/// The state of the [`SampleReceiver`]
pub enum SampleReceiverWaitState {
    /// Samples are available and can be taken from the queue by [`SampleReceiver::take`]
    SamplesAvailable,
    /// No samples received during the wait duration
    Timeout,
    /// The [`SampleReceiver`] was stopped
    Stopped,
}
