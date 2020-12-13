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
    pub fn new(service: &str, instance: &str, event: &str, history_capacity: u64) -> Result<Self, IceOryxError> {
        let ffi_pub = FfiPublisher::new(service, instance, event, history_capacity);

        Ok(Topic {
            ffi_pub: ffi_pub.ok_or(IceOryxError::PublisherTopicCreationFailed)?,
            phantom: PhantomData,
        })
    }

    pub fn offer(self) -> Publisher<T> {
        self.ffi_pub.offer();
        Publisher::new(self)
    }
}

impl<T: POD> Drop for Topic<T> {
    fn drop(&mut self) {
        self.ffi_pub.stop_offer();
    }
}
