// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: © Contributors to the iceoryx-rs project
// SPDX-FileContributor: Mathias Kraus

use crate::SubscribeState;

use std::marker::PhantomData;
use std::slice::from_raw_parts_mut;
use std::time::{Duration, SystemTime};

use std::ops::Deref;

//TODO impl debug for Sample with T: Debug
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

pub struct SampleReceiver<T: ?Sized, S: ffi::SubscriberStrongRef> {
    ffi_sub: S,
    pub condition_variable: Box<ffi::ConditionVariable>,
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

    pub fn subscription_state(&self) -> SubscribeState {
        self.ffi_sub.as_ref().subscription_state()
    }

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

    pub fn has_data(&self) -> bool {
        self.ffi_sub.as_ref().has_chunks()
    }

    pub fn clear(&self) {
        self.ffi_sub.as_ref().clear();
    }
}

impl<T, S: ffi::SubscriberStrongRef> SampleReceiver<T, S> {
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
    pub fn take(&self) -> Option<Sample<[T], S>> {
        self.ffi_sub
            .as_ref()
            .get_chunk()
            .and_then(|data: *const T| {
                let payload_size = self.ffi_sub.as_ref().get_user_payload_size(data);
                let len = payload_size as usize / std::mem::size_of::<T>();

                if payload_size as usize % std::mem::size_of::<T>() == 0 {
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

pub enum SampleReceiverWaitState {
    SamplesAvailable,
    Timeout,
    Stopped,
}
