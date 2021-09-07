// SPDX-License-Identifier: Apache-2.0

use super::{ffi, SubscribeState};

use std::marker::PhantomData;
use std::time::{Duration, SystemTime};

use std::ops::Deref;

//TODO impl debug for Sample with T: Debug
pub struct Sample<T, S: ffi::SubscriberStrongRef> {
    pub data: Option<Box<T>>,
    ffi_sub: S,
}

impl<T, S: ffi::SubscriberStrongRef> Deref for Sample<T, S> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.data.as_ref().expect("valid sample")
    }
}

impl<T, S: ffi::SubscriberStrongRef> Drop for Sample<T, S> {
    fn drop(&mut self) {
        if let Some(chunk) = self.data.take() {
            self.ffi_sub.as_ref().release_chunk(chunk);
        }
    }
}

pub struct SampleReceiver<T, S: ffi::SubscriberStrongRef> {
    ffi_sub: S,
    pub condition_variable: Box<ffi::ConditionVariable>,
    phantom: PhantomData<T>,
}

impl<T, S: ffi::SubscriberStrongRef> SampleReceiver<T, S> {
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
        if self.has_samples() {
            return SampleReceiverWaitState::SamplesAvailable;
        }

        let entry_time = SystemTime::now();
        while let Some(remaining_timeout) = {
            let elapsed = entry_time.elapsed().unwrap_or(timeout);
            timeout.checked_sub(elapsed)
        } {
            self.condition_variable.timed_wait(remaining_timeout);
            if self.has_samples() {
                return SampleReceiverWaitState::SamplesAvailable;
            }
        }

        if self.ffi_sub.as_ref().is_condition_variable_set() {
            SampleReceiverWaitState::Timeout
        } else {
            SampleReceiverWaitState::Stopped
        }
    }

    pub fn has_samples(&self) -> bool {
        self.ffi_sub.as_ref().has_chunks()
    }

    pub fn clear(&self) {
        self.ffi_sub.as_ref().clear();
    }

    pub fn get_sample(&self) -> Option<Sample<T, S>> {
        self.ffi_sub
            .as_ref()
            .get_chunk()
            .map(|chunk| Sample::<T, S> {
                data: Some(chunk),
                ffi_sub: self.ffi_sub.clone(),
            })
    }
}

impl<T, S: super::ffi::SubscriberStrongRef> Drop for SampleReceiver<T, S> {
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
