// Copyright 2020 Mathias Kraus - All rights reserved
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// http://www.apache.org/licenses/LICENSE-2.0>. This file may not be
// copied, modified, or distributed except according to those terms.

mod ffi;
mod recipient;
mod sample;

pub use ffi::SubscriptionState;
pub use sample::Sample;
pub use sample::SampleReceiverWaitState;

use recipient::{RecipientMT, RecipientST};

use std::marker::PhantomData;

pub struct SampleReceiverToken {}

pub struct Subscriber<T> {
    ffi_sub: Box<ffi::Subscriber>,
    phantom: PhantomData<T>,
}

impl<T> Subscriber<T> {
    pub fn new(service: &str, instance: &str, event: &str) -> Self {
        Self {
            ffi_sub: ffi::Subscriber::new(service, instance, event),
            phantom: PhantomData,
        }
    }

    pub fn subscribe(self, cache_size: u32) -> (RecipientST<T>, SampleReceiverToken) {
        self.ffi_sub.subscribe(cache_size);
        (RecipientST::new(self), SampleReceiverToken {})
    }

    pub fn subscribe_mt(self, cache_size: u32) -> (RecipientMT<T>, SampleReceiverToken) {
        self.ffi_sub.subscribe(cache_size);
        (RecipientMT::new(self), SampleReceiverToken {})
    }

    pub fn subscription_state(&self) -> SubscriptionState {
        self.ffi_sub.subscription_state()
    }
}
