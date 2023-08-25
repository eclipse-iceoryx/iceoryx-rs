// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: © Contributors to the iceoryx-rs project
// SPDX-FileContributor: Mathias Kraus

use crate::marker::ShmSend;
use crate::SubscribeState;

use std::marker::PhantomData;
use std::slice::from_raw_parts_mut;
use std::time::{Duration, SystemTime};

use std::ops::Deref;

//TODO impl debug for Sample with T: Debug
/// An immutable sample shared between multiple subscriber
pub struct Sample<T: ?Sized, S: ffi::SubscriberStrongRef> {
    data: Option<Box<T>>,
    ffi_sub: S,
}

impl<T: ?Sized, S: ffi::SubscriberStrongRef> Deref for Sample<T, S> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // this is safe since only `drop` will take from the `Option`
        unsafe { self.data.as_ref().unwrap_unchecked() }
    }
}

impl<T: ?Sized, S: ffi::SubscriberStrongRef> Drop for Sample<T, S> {
    fn drop(&mut self) {
        if let Some(chunk) = self.data.take() {
            self.ffi_sub.as_ref().release_chunk(Box::into_raw(chunk));
        }
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
        let data = self.data.as_ref().unwrap_unchecked();
        let chunk_header =
            ffi::ChunkHeader::from_user_payload(&*(data.as_ptr())).unwrap_unchecked();
        let payload_size = chunk_header.get_user_payload_size() as usize;
        let payload_alignment = chunk_header.get_user_payload_alignment() as usize;

        if payload_size >= std::mem::size_of::<T>()
            && payload_alignment >= std::mem::align_of::<T>()
        {
            Some(&*(std::mem::transmute::<*const u8, *const T>(data.as_ptr())))
        } else {
            None
        }
    }
}

/// Access to the sample receiver queue of the subscriber
pub struct SampleReceiver<T: ?Sized, S: ffi::SubscriberStrongRef> {
    ffi_sub: S,
    // condition_variable: Box<ffi::ConditionVariable>,
    phantom: PhantomData<T>,
}

use crate::reactor::Foo;

use std::any::Any;

impl<T: ?Sized + 'static, S: ffi::SubscriberStrongRef + 'static> Foo for SampleReceiver<T, S> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn attach_condition_variable(&self, condition_variable: &ffi::ConditionVariable, notification_index: u64) {
        self.ffi_sub.as_ref().set_condition_variable(&condition_variable, notification_index);
    }
}

impl<T: ?Sized, S: ffi::SubscriberStrongRef> SampleReceiver<T, S> {
    pub(super) fn new(ffi_sub: S) -> Self {
        // let condition_variable = ffi::ConditionVariable::new();
        // // currently the condition variable is used only for one subscriber and therefore the index is set to 0
        // let notification_index = 0;
        // ffi_sub.as_ref().set_condition_variable(&condition_variable, notification_index);

        SampleReceiver {
            ffi_sub,
            // condition_variable,
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
            // self.condition_variable.timed_wait(remaining_timeout);
            // if self.has_data() {
            //     return SampleReceiverWaitState::SamplesAvailable;
            // }
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
}

impl<T, S: ffi::SubscriberStrongRef> SampleReceiver<T, S> {
    /// Takes a sample from the receiver queue
    ///
    /// If the receiver queue is empty, `None` will be returned.
    pub fn take(&self) -> Option<Sample<T, S>> {
        self.ffi_sub.as_ref().get_chunk().map(|data: *const T| {
            // this is safe since sample only implements `Deref` and not `DerefMut`
            let data = unsafe { Box::from_raw(data as *mut T) };
            Sample::<T, S> {
                data: Some(data),
                ffi_sub: self.ffi_sub.clone(),
            }
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
            .get_chunk()
            .and_then(|data: *const T| {
                let chunk_header =
                    unsafe { ffi::ChunkHeader::from_user_payload(&*data).unwrap_unchecked() };
                let payload_size = chunk_header.get_user_payload_size();
                let payload_alignment = chunk_header.get_user_payload_alignment();
                let len = payload_size as usize / std::mem::size_of::<T>();

                if payload_size as usize % std::mem::size_of::<T>() == 0
                    && payload_alignment as usize >= std::mem::align_of::<T>()
                {
                    let data = unsafe {
                        // this is safe since sample only implements `Deref` and not `DerefMut`
                        let data = from_raw_parts_mut(data as *mut T, len as usize);
                        Box::from_raw(data)
                    };

                    Some(Sample::<[T], S> {
                        data: Some(data),
                        ffi_sub: self.ffi_sub.clone(),
                    })
                } else {
                    // TODO return Result<Option<T>>
                    self.ffi_sub.as_ref().release_chunk(data);
                    None
                }
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
