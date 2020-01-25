// Copyright 2020 Mathias Kraus - All rights reserved
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// http://www.apache.org/licenses/LICENSE-2.0>. This file may not be
// copied, modified, or distributed except according to those terms.

use super::{ffi, ffi::SubscriptionState};
use super::{mt, st};

use std::marker::PhantomData;

pub struct SampleReceiverToken {}

pub struct Topic<T> {
    pub(super) ffi_sub: Box<ffi::Subscriber>,
    phantom: PhantomData<T>,
}

impl<T> Topic<T> {
    pub fn new(service: &str, instance: &str, event: &str) -> Self {
        Topic {
            ffi_sub: ffi::Subscriber::new(service, instance, event),
            phantom: PhantomData,
        }
    }

    pub(super) fn from_ffi(ffi_sub: Box<ffi::Subscriber>) -> Self {
        Topic {
            ffi_sub,
            phantom: PhantomData,
        }
    }

    pub fn subscribe(self, cache_size: u32) -> (st::Subscriber<T>, SampleReceiverToken) {
        self.ffi_sub.subscribe(cache_size);
        (st::Subscriber::new(self), SampleReceiverToken {})
    }

    pub fn subscribe_mt(self, cache_size: u32) -> (mt::Subscriber<T>, SampleReceiverToken) {
        self.ffi_sub.subscribe(cache_size);
        (mt::Subscriber::new(self), SampleReceiverToken {})
    }

    pub fn subscription_state(&self) -> SubscriptionState {
        self.ffi_sub.subscription_state()
    }
}
