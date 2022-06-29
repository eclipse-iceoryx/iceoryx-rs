// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: © Contributors to the iceoryx-rs project
// SPDX-FileContributor: Mathias Kraus

use super::sample::SampleReceiver;
use super::{mt, st};
use crate::IceoryxError;
use crate::QueueFullPolicy;
use crate::SubscribeState;

use std::marker::PhantomData;

pub struct SubscriberBuilder<'a, T> {
    service: &'a str,
    instance: &'a str,
    event: &'a str,
    options: ffi::SubscriberOptions,
    phantom: PhantomData<T>,
}

impl<'a, T> SubscriberBuilder<'a, T> {
    pub fn new(service: &'a str, instance: &'a str, event: &'a str) -> Self {
        Self {
            service,
            instance,
            event,
            options: ffi::SubscriberOptions::default(),
            phantom: PhantomData,
        }
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

    pub fn queue_full_policy(mut self, queue_full_policy: QueueFullPolicy) -> Self {
        self.options.queue_full_policy = queue_full_policy;
        self
    }

    pub fn requires_publisher_history_support(
        mut self,
        requires_publisher_history_support: bool,
    ) -> Self {
        self.options.requires_publisher_history_support = requires_publisher_history_support;
        self
    }

    pub fn create(mut self) -> Result<(st::Subscriber<T>, SampleReceiverToken), IceoryxError> {
        self.options.subscribe_on_create = true;
        let ffi_sub = ffi::Subscriber::new(self.service, self.instance, self.event, &self.options)
            .ok_or(IceoryxError::SubscriberCreationFailed)?;

        let subscriber = st::Subscriber {
            ffi_sub: ffi::SubscriberRc::new(ffi_sub),
            phantom: PhantomData,
        };

        Ok((subscriber, SampleReceiverToken {}))
    }

    pub fn create_mt(mut self) -> Result<(mt::Subscriber<T>, SampleReceiverToken), IceoryxError> {
        self.options.subscribe_on_create = true;
        let ffi_sub = ffi::Subscriber::new(self.service, self.instance, self.event, &self.options)
            .ok_or(IceoryxError::SubscriberCreationFailed)?;

        let subscriber = mt::Subscriber {
            ffi_sub: ffi::SubscriberArc::new(ffi_sub),
            phantom: PhantomData,
        };

        Ok((subscriber, SampleReceiverToken {}))
    }

    pub fn create_without_subscribe(mut self) -> Result<InactiveSubscriber<T>, IceoryxError> {
        self.options.subscribe_on_create = false;
        let ffi_sub = ffi::Subscriber::new(self.service, self.instance, self.event, &self.options)
            .ok_or(IceoryxError::SubscriberCreationFailed)?;

        Ok(InactiveSubscriber {
            ffi_sub,
            phantom: PhantomData,
        })
    }
}

pub struct SampleReceiverToken {}

pub struct InactiveSubscriber<T> {
    ffi_sub: Box<ffi::Subscriber>,
    phantom: PhantomData<T>,
}

impl<T> InactiveSubscriber<T> {
    fn from_ffi(ffi_sub: Box<ffi::Subscriber>) -> Self {
        Self {
            ffi_sub,
            phantom: PhantomData,
        }
    }

    pub fn subscribe(self) -> (st::Subscriber<T>, SampleReceiverToken) {
        self.ffi_sub.subscribe();
        (
            st::Subscriber::new_from_ffi(self.ffi_sub),
            SampleReceiverToken {},
        )
    }

    pub fn subscribe_mt(self) -> (mt::Subscriber<T>, SampleReceiverToken) {
        self.ffi_sub.subscribe();
        (
            mt::Subscriber::new_from_ffi(self.ffi_sub),
            SampleReceiverToken {},
        )
    }

    pub fn subscription_state(&self) -> SubscribeState {
        self.ffi_sub.subscription_state()
    }
}

pub struct Subscriber<T, S: ffi::SubscriberStrongRef> {
    ffi_sub: S,
    phantom: PhantomData<T>,
}

impl<T, S: ffi::SubscriberStrongRef> Subscriber<T, S> {
    fn new_from_ffi(ffi_sub: Box<ffi::Subscriber>) -> Self {
        Subscriber {
            ffi_sub: S::new(ffi_sub),
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

    pub fn unsubscribe(self, sample_receiver: SampleReceiver<T, S>) -> InactiveSubscriber<T> {
        self.ffi_sub.as_ref().unsubscribe();

        drop(sample_receiver);

        InactiveSubscriber::from_ffi(self.ffi_sub.take())
    }
}
