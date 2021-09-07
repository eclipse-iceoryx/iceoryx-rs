// SPDX-License-Identifier: Apache-2.0

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
        SampleReceiver::<T, S>::new(self.ffi_sub.clone())
    }

    pub fn stop_sample_receiver(&self) {
        self.ffi_sub.as_ref().unset_condition_variable();
    }

    pub fn unsubscribe(self, sample_receiver: SampleReceiver<T, S>) -> Topic<T> {
        self.ffi_sub.as_ref().unsubscribe();

        drop(sample_receiver);

        Topic::from_ffi(self.ffi_sub.take())
    }
}
