// Copyright 2020 Mathias Kraus - All rights reserved
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// http://www.apache.org/licenses/LICENSE-2.0>. This file may not be
// copied, modified, or distributed except according to those terms.

use super::{ffi, SubscriptionState};

use std::marker::PhantomData;
use std::time::{Duration, SystemTime};

use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;

pub struct Sample<'a, T> {
    pub(super) data: Option<Box<T>>,
    pub(super) ffi_sub: &'a ffi::Subscriber,
}

impl<'a, T> Deref for Sample<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.data.as_ref().expect("valid sample")
    }
}

impl<'a, T> Drop for Sample<'a, T> {
    fn drop(&mut self) {
        if let Some(chunk) = self.data.take() {
            self.ffi_sub.release_chunk(chunk)
        }
    }
}

pub struct SampleReceiverST<T> {
    ffi_sub: Rc<Box<ffi::Subscriber>>,
    phantom: PhantomData<T>,
}

impl<T> SampleReceiverST<T> {
    pub(super) fn new(ffi_sub: Rc<Box<ffi::Subscriber>>) -> Self {
        Self {
            ffi_sub,
            phantom: PhantomData,
        }
    }

    pub fn subscription_state(&self) -> SubscriptionState {
        self.ffi_sub.subscription_state()
    }

    pub fn has_samples(&self) -> bool {
        self.ffi_sub.has_chunks()
    }

    pub fn clear(&self) {
        self.ffi_sub.clear();
    }

    pub fn get_sample(&self) -> Option<Sample<T>> {
        self.ffi_sub.get_chunk().map(|chunk| Sample {
            data: Some(chunk),
            ffi_sub: &self.ffi_sub,
        })
    }
}

impl<T> Drop for SampleReceiverST<T> {
    fn drop(&mut self) {
        self.ffi_sub.unsubscribe();
    }
}

pub enum SampleReceiverWaitState {
    SamplesAvailable,
    Timeout,
    Stopped,
}

pub struct SampleReceiverMT<T> {
    ffi_sub: Arc<Box<ffi::Subscriber>>,
    phantom: PhantomData<T>,
}

impl<T> SampleReceiverMT<T> {
    pub(super) fn new(ffi_sub: Arc<Box<ffi::Subscriber>>) -> Self {
        Self {
            ffi_sub,
            phantom: PhantomData,
        }
    }

    pub fn subscription_state(&self) -> SubscriptionState {
        self.ffi_sub.subscription_state()
    }

    pub fn wait_for_samples(&self, timeout: Duration) -> SampleReceiverWaitState {
        if !self.ffi_sub.wait_for_chunks_enabled() {
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
            self.ffi_sub.wait_for_chunks(remaining_timeout);
            if self.has_samples() {
                return SampleReceiverWaitState::SamplesAvailable;
            }
        }

        if self.ffi_sub.wait_for_chunks_enabled() {
            SampleReceiverWaitState::Timeout
        } else {
            SampleReceiverWaitState::Stopped
        }
    }

    pub fn has_samples(&self) -> bool {
        self.ffi_sub.has_chunks()
    }

    pub fn clear(&self) {
        self.ffi_sub.clear();
    }

    pub fn get_sample(&self) -> Option<Sample<T>> {
        self.ffi_sub.get_chunk().map(|chunk| Sample {
            data: Some(chunk),
            ffi_sub: &self.ffi_sub,
        })
    }
}

impl<T> Drop for SampleReceiverMT<T> {
    fn drop(&mut self) {
        self.ffi_sub.unsubscribe();
    }
}
