// Copyright 2020 Mathias Kraus - All rights reserved
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// http://www.apache.org/licenses/LICENSE-2.0>. This file may not be
// copied, modified, or distributed except according to those terms.

use super::{ffi::Publisher as FfiPublisher, Publisher, POD};
use crate::IceOryxError;

use std::marker::PhantomData;

pub struct Topic<T: POD> {
    pub(super) ffi_pub: Box<FfiPublisher>,
    phantom: PhantomData<T>,
}

impl<T: POD> Topic<T> {
    pub fn new(service: &str, instance: &str, event: &str) -> Self {
        Topic {
            ffi_pub: FfiPublisher::new(service, instance, event),
            phantom: PhantomData,
        }
    }

    // TODO create a disableDoDeliveryOnSubscription() in iceory and
    // change this to an offer_with_delivery_on_subscription()
    pub fn new_with_delivery_on_subscription(
        service: &str,
        instance: &str,
        event: &str,
        data: T,
    ) -> Result<Self, IceOryxError> {
        let ffi_pub = FfiPublisher::new(service, instance, event);

        ffi_pub.enable_delivery_on_subscription();

        let mut chunk = ffi_pub.allocate_chunk::<T>()?;
        *chunk = data;
        ffi_pub.send_chunk::<T>(chunk);

        Ok(Topic {
            ffi_pub,
            phantom: PhantomData,
        })
    }

    pub fn offer(self) -> Publisher<T> {
        // TODO since the RouDi discovery loop introduces a latency until the service is offered,
        // a OfferState, similar to the SubscriptionState might be a worthwhile idea
        self.ffi_pub.offer();
        Publisher::new(self)
    }
}

impl<T: POD> Drop for Topic<T> {
    fn drop(&mut self) {
        self.ffi_pub.stop_offer();
    }
}
