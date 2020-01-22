// Copyright 2020 Mathias Kraus - All rights reserved
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// http://www.apache.org/licenses/LICENSE-2.0>. This file may not be
// copied, modified, or distributed except according to those terms.

use super::{
    ffi,
    sample::{SampleReceiverMT, SampleReceiverST},
    topic::SampleReceiverToken,
    SubscriptionState, Topic,
};

use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::Arc;

pub struct SubscriberST<T> {
    ffi_sub: Rc<Box<ffi::Subscriber>>,
    phantom: PhantomData<T>,
}

pub struct SubscriberMT<T> {
    ffi_sub: Arc<Box<ffi::Subscriber>>,
    phantom: PhantomData<T>,
}

impl<T> SubscriberST<T> {
    pub(super) fn new(subscriber: Topic<T>) -> Self {
        SubscriberST {
            ffi_sub: Rc::new(subscriber.ffi_sub),
            phantom: PhantomData,
        }
    }

    pub fn subscription_state(&self) -> SubscriptionState {
        self.ffi_sub.subscription_state()
    }

    pub fn get_sample_receiver(&self, _: SampleReceiverToken) -> SampleReceiverST<T> {
        self.ffi_sub.enable_wait_for_chunks();
        SampleReceiverST::new(self.ffi_sub.clone())
    }

    pub fn unsubscribe(self, sample_receiver: SampleReceiverST<T>) -> Topic<T> {
        // TODO once ffi::Subscriber::disable_wait_for_chunks() is available, call it here
        self.ffi_sub.unsubscribe();

        drop(sample_receiver);

        Topic::from_ffi(Rc::try_unwrap(self.ffi_sub).expect("Unique owner of subscriber"))
    }
}

impl<T> SubscriberMT<T> {
    pub(super) fn new(subscriber: Topic<T>) -> Self {
        SubscriberMT {
            ffi_sub: Arc::new(subscriber.ffi_sub),
            phantom: PhantomData,
        }
    }

    pub fn subscription_state(&self) -> SubscriptionState {
        self.ffi_sub.subscription_state()
    }

    pub fn get_sample_receiver(&self, _: SampleReceiverToken) -> SampleReceiverMT<T> {
        self.ffi_sub.enable_wait_for_chunks();
        SampleReceiverMT::new(self.ffi_sub.clone())
    }

    // TODO once ffi::Subscriber::disable_wait_for_chunks() is available, enable this
    // pub fn stop_sample_receiver(&self) {
    //     self.ffi_sub.disable_wait_for_chunks();
    // }

    pub fn unsubscribe(self, sample_receiver: SampleReceiverMT<T>) -> Topic<T> {
        // TODO once ffi::Subscriber::disable_wait_for_chunks() is available, call it here
        self.ffi_sub.unsubscribe();

        drop(sample_receiver);

        Topic::from_ffi(Arc::try_unwrap(self.ffi_sub).expect("Unique owner of subscriber"))
    }
}

// // TODO this should probably be in subscriber.rs
// trait SingleThreaded {}
// trait MultiThreaded {}
//
// impl <T> MultiThreaded for SubscriberMT<T> {}
//
// pub trait SubscriberTrait<'a, T> {
//     type SampleReceiver;
//
//     fn subscription_state(&self) -> SubscriptionState;
//
//     fn get_sample_receiver(&'a self, _: SampleReceiverToken) -> Self::SampleReceiver;
//
//     fn unsubscribe(self) -> Subscriber<T>;
// }
//
// impl<'a, T> SubscriberTrait<'a, T> for SubscriberST<T> {
//     type SampleReceiver = SampleReceiverST<'a, T>;
//
//     fn subscription_state(&self) -> SubscriptionState {
//         self.ffi_sub.subscription_state()
//     }
//
//     fn get_sample_receiver(&'a self, _: SampleReceiverToken) -> Self::SampleReceiver {
//         self.ffi_sub.enable_wait_for_chunks();
//         SampleReceiverST::new(&self.ffi_sub)
//     }
//
//     // TODO consumes a SampleReceiver or SampleEventReceiver (trait bound) and returns a Subscriber
//     // Subscriber, consumes a SampleReceiver ... maybe
//     fn unsubscribe(self) -> Subscriber<T> {
//         // TODO once ffi::Subscriber::disable_wait_for_chunks() is available, call it here
//         self.ffi_sub.unsubscribe();
//
//         Subscriber {
//             ffi_sub: self.ffi_sub,
//             phantom: PhantomData,
//         }
//     }
// }
