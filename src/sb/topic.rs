// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: © Contributors to the iceoryx-rs project
// SPDX-FileContributor: Mathias Kraus

use super::{ffi, ffi::SubscribeState, SubscriberOptions};
use super::{mt, st};

use std::marker::PhantomData;

pub struct TopicBuilder<'a, T> {
    service: &'a str,
    instance: &'a str,
    event: &'a str,
    options: SubscriberOptions,
    phantom: PhantomData<T>,
}

impl<'a, T> TopicBuilder<'a, T> {
    pub fn new(service: &'a str, instance: &'a str, event: &'a str) -> Self {
        Self {service, instance, event, options: SubscriberOptions::default(), phantom: PhantomData}
    }

    pub fn queue_capacity(mut self, queue_capacity: u64) -> Self {
        self.options.queue_capacity = queue_capacity;
        self
    }

    pub fn history_request(mut self, history_request: u64) -> Self {
        self.options.history_request = history_request;
        self
    }

    pub fn node_name(mut self, node_name: String) -> Self {
        self.options.node_name = node_name;
        self
    }

    pub fn subscribe_on_create(mut self, subscribe_on_create: bool) -> Self {
        self.options.subscribe_on_create = subscribe_on_create;
        self
    }

    pub fn build(self) ->  Topic<T> {
        Topic {
            ffi_sub: ffi::Subscriber::new(self.service, self.instance, self.event, &self.options),
            phantom: PhantomData,
        }
    }
}

pub struct SampleReceiverToken {}

pub struct Topic<T> {
    pub(super) ffi_sub: Box<ffi::Subscriber>,
    phantom: PhantomData<T>,
}

impl<T> Topic<T> {
    pub(super) fn from_ffi(ffi_sub: Box<ffi::Subscriber>) -> Self {
        Topic {
            ffi_sub,
            phantom: PhantomData,
        }
    }

    pub fn subscribe(self) -> (st::Subscriber<T>, SampleReceiverToken) {
        self.ffi_sub.subscribe();
        (st::Subscriber::new(self), SampleReceiverToken {})
    }

    pub fn subscribe_mt(self) -> (mt::Subscriber<T>, SampleReceiverToken) {
        self.ffi_sub.subscribe();
        (mt::Subscriber::new(self), SampleReceiverToken {})
    }

    pub fn subscription_state(&self) -> SubscribeState {
        self.ffi_sub.subscription_state()
    }
}
