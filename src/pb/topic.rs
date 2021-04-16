// SPDX-License-Identifier: Apache-2.0

use super::{ffi::Publisher as FfiPublisher, Publisher, PublisherOptions, POD};
use crate::IceOryxError;

use std::marker::PhantomData;

pub struct TopicBuilder<'a, T: POD> {
    service: &'a str,
    instance: &'a str,
    event: &'a str,
    options: PublisherOptions,
    phantom: PhantomData<T>,
}

impl<'a, T: POD> TopicBuilder<'a, T> {
    pub fn new(service: &'a str, instance: &'a str, event: &'a str) -> Self {
        Self {service, instance, event, options: PublisherOptions::default(), phantom: PhantomData}
    }

    pub fn history_capacity(mut self, history_capacity: u64) -> Self {
        self.options.history_capacity = history_capacity;
        self
    }

    pub fn node_name(mut self, node_name: String) -> Self {
        self.options.node_name = node_name;
        self
    }

    pub fn offer_on_create(mut self, offer_on_create: bool) -> Self {
        self.options.offer_on_create = offer_on_create;
        self
    }

    pub fn build(self) ->  Result<Topic<T>, IceOryxError> {
        let ffi_pub = FfiPublisher::new(self.service, self.instance, self.event, &self.options);

        Ok(Topic {
            ffi_pub: ffi_pub.ok_or(IceOryxError::PublisherTopicCreationFailed)?,
            phantom: PhantomData,
        })
    }
}

pub struct Topic<T: POD> {
    pub(super) ffi_pub: Box<FfiPublisher>,
    phantom: PhantomData<T>,
}

impl<T: POD> Topic<T> {
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
