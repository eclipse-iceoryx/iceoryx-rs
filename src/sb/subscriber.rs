// Copyright 2020 Mathias Kraus - All rights reserved
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// http://www.apache.org/licenses/LICENSE-2.0>. This file may not be
// copied, modified, or distributed except according to those terms.

use super::{
    ffi::SubscriberStrongRef, sample::SampleReceiver, topic::SampleReceiverToken, SubscribeState,
    Topic,
};

use std::marker::PhantomData;

pub struct Subscriber<T, S: SubscriberStrongRef> {
    ffi_sub: S,
    phantom: PhantomData<T>,
}

impl<T, S: SubscriberStrongRef> Subscriber<T, S> {
    pub(super) fn new(subscriber: Topic<T>) -> Self {
        Subscriber {
            ffi_sub: S::new(subscriber.ffi_sub),
            phantom: PhantomData,
        }
    }

    pub fn subscription_state(&self) -> SubscribeState {
        self.ffi_sub.as_ref().subscription_state()
    }

    pub fn get_sample_receiver(&self, _: SampleReceiverToken) -> SampleReceiver<T, S> {
        self.ffi_sub.as_ref().enable_wait_for_chunks();
        SampleReceiver::<T, S>::new(self.ffi_sub.clone())
    }

    // TODO once ffi::Subscriber::disable_wait_for_chunks() is available, enable this
    // pub fn stop_sample_receiver(&self) {
    //     self.ffi_sub.disable_wait_for_chunks();
    // }

    pub fn unsubscribe(self, sample_receiver: SampleReceiver<T, S>) -> Topic<T> {
        // TODO once ffi::Subscriber::disable_wait_for_chunks() is available, call it here
        self.ffi_sub.as_ref().unsubscribe();

        drop(sample_receiver);

        Topic::from_ffi(self.ffi_sub.take())
    }
}
