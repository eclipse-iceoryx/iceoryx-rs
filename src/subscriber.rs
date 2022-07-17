// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: © Contributors to the iceoryx-rs project
// SPDX-FileContributor: Mathias Kraus

use super::sample::SampleReceiver;
use super::{mt, st};
use crate::IceoryxError;
use crate::QueueFullPolicy;
use crate::SubscribeState;

use std::marker::PhantomData;

/// Create a subscriber with custom options
///
/// # Example
/// ```
/// # use iceoryx_rs::Runtime;
/// use iceoryx_rs::SubscriberBuilder;
/// # use ffi::RouDiEnvironment;
/// #
/// # use anyhow::{anyhow, Result};
/// # fn main() -> Result<()> {
/// # let _roudi = RouDiEnvironment::new();
/// #
/// # Runtime::init("basic_pub_sub");
///
/// let (subscriber, sample_receive_token) =
///     SubscriberBuilder::<u32>::new("all", "glory", "hynotoad")
///         .queue_capacity(10)
///         .history_request(5)
///         .create()?;
/// # Ok(())
/// # }
/// ```
pub struct SubscriberBuilder<'a, T: ?Sized> {
    service: &'a str,
    instance: &'a str,
    event: &'a str,
    options: ffi::SubscriberOptions,
    phantom: PhantomData<T>,
}

impl<'a, T: ?Sized> SubscriberBuilder<'a, T> {
    /// Creates a new `SubscriberBuilder`
    ///
    /// The parameter `service`, `instance` and `event` are used to specify the name of the service.
    /// In the future this three strings will probably be fused to together in a single string separated
    /// by slashes `/`.
    pub fn new(service: &'a str, instance: &'a str, event: &'a str) -> Self {
        Self {
            service,
            instance,
            event,
            options: ffi::SubscriberOptions::default(),
            phantom: PhantomData,
        }
    }

    /// The size of the receiver queue where samples are stored before they are passed to the user
    pub fn queue_capacity(mut self, size: u64) -> Self {
        self.options.queue_capacity = size;
        self
    }

    /// The max number of samples received after subscription if samples are available
    pub fn history_request(mut self, max_number_of_samples: u64) -> Self {
        self.options.history_request = max_number_of_samples;
        self
    }

    /// The name of the node where the subscriber should belong to
    ///
    /// Setting the node name has currently no functionality but this might change in future.
    pub fn node_name(mut self, name: &str) -> Self {
        self.options.node_name = name.to_string();
        self
    }

    /// Set the behavior of full receiver queue
    ///
    /// By default the publisher discards the oldest data to make room for new data. Optionally,
    /// the publisher can be asked to block. This only works if the publisher has matching options.
    ///
    /// While it is possible to block the publisher, it is not recommended to use this option since
    /// other subscriber to the publisher will also be affected and don't receive samples until
    /// the subscriber with the full queue takes samples out of the queue.
    pub fn queue_full_policy(mut self, queue_full_policy: QueueFullPolicy) -> Self {
        self.options.queue_full_policy = queue_full_policy;
        self
    }

    /// Indicates whether to enforce history support of the publisher, i.e. require
    /// `PublisherOptions::historyCapacity > 0` to be eligible to be connected
    pub fn requires_publisher_history_support(mut self, flag: bool) -> Self {
        self.options.requires_publisher_history_support = flag;
        self
    }

    /// Creates a new [`st::Subscriber`] which is restricted to be used single-threaded
    ///
    /// When this method returns and there is a corresponding publisher, the subscriber is immediately connected
    /// and if the publisher has buffered samples, they are received according to the requested history.
    ///
    /// # Panics
    ///
    /// [`Runtime::init`](crate::Runtime::init) must have been called otherwise this method will panic.
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

    /// Creates a new [`mt::Subscriber`] which is capable to be used multi-threaded
    ///
    /// When this method returns and there is a corresponding publisher, the subscriber is immediately connected
    /// and if the publisher has buffered samples, they are received according to the requested history.
    ///
    /// # Panics
    ///
    /// [`Runtime::init`](crate::Runtime::init) must have been called otherwise this method will panic.
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

    /// Creates a new [`InactiveSubscriber`]
    ///
    /// The subscriber is not subscribed and did not request to subscribe to a publisher. No samples will be received,
    /// even if there are matching publisher which are in the offer state.
    ///
    /// # Panics
    ///
    /// [`Runtime::init`](crate::Runtime::init) must have been called otherwise this method will panic.
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

/// An inactive subscriber which is not subscribed and is not visible to any publisher
///
/// This can be used for cases where the service is suspended and the publisher/subscriber need to be disconnected.
pub struct InactiveSubscriber<T: ?Sized> {
    ffi_sub: Box<ffi::Subscriber>,
    phantom: PhantomData<T>,
}

impl<T: ?Sized> InactiveSubscriber<T> {
    fn from_ffi(ffi_sub: Box<ffi::Subscriber>) -> Self {
        Self {
            ffi_sub,
            phantom: PhantomData,
        }
    }

    /// Subscribes to a publisher by consuming the `InactiveSubscriber` and creating a [`st::Subscriber`]
    ///
    /// It might take up to 50 milliseconds until `RouDi` runs its discovery loop and the subscriber
    /// will be subscribed to the publisher.
    pub fn subscribe(self) -> (st::Subscriber<T>, SampleReceiverToken) {
        self.ffi_sub.subscribe();
        (
            st::Subscriber::new_from_ffi(self.ffi_sub),
            SampleReceiverToken {},
        )
    }

    /// Subscribes to a publisher by consuming the `InactiveSubscriber` and creating a [`mt::Subscriber`]
    ///
    /// It might take up to 50 milliseconds until `RouDi` runs its discovery loop and the subscriber
    pub fn subscribe_mt(self) -> (mt::Subscriber<T>, SampleReceiverToken) {
        self.ffi_sub.subscribe();
        (
            mt::Subscriber::new_from_ffi(self.ffi_sub),
            SampleReceiverToken {},
        )
    }

    /// The current subscription state
    ///
    /// After [`SubscriberBuilder::create`] this will immediately be [`SubscribeState::Subscribed`] but after
    /// [`InactiveSubscriber::subscribe`] it might take up to 50 milliseconds until `RouDi` runs its
    /// discovery loop and the subscriber will be subscribed to the publisher.
    pub fn subscription_state(&self) -> SubscribeState {
        self.ffi_sub.subscription_state()
    }
}

/// A subscriber which is subscribed or requested to be subscribed to a publisher
pub struct Subscriber<T: ?Sized, S: ffi::SubscriberStrongRef> {
    ffi_sub: S,
    phantom: PhantomData<T>,
}

impl<T: ?Sized, S: ffi::SubscriberStrongRef> Subscriber<T, S> {
    fn new_from_ffi(ffi_sub: Box<ffi::Subscriber>) -> Self {
        Subscriber {
            ffi_sub: S::new(ffi_sub),
            phantom: PhantomData,
        }
    }

    /// The current subscription state
    ///
    /// After [`SubscriberBuilder::create`] this will immediately be [`SubscribeState::Subscribed`] but after
    /// [`InactiveSubscriber::subscribe`] it might take up to 50 milliseconds until `RouDi` runs its
    /// discovery loop and the subscriber will be subscribed to the publisher.
    pub fn subscription_state(&self) -> SubscribeState {
        self.ffi_sub.as_ref().subscription_state()
    }

    /// Obtain the [`SampleReceiver`] to take samples published by a publisher
    pub fn get_sample_receiver(&self, _: SampleReceiverToken) -> SampleReceiver<T, S> {
        SampleReceiver::<T, S>::new(self.ffi_sub.clone())
    }

    /// Obtain the [`SampleReceiver`] to take samples published by a publisher
    pub fn stop_sample_receiver(&self) {
        self.ffi_sub.as_ref().unset_condition_variable();
    }

    /// Unsubscribes from the publisher by consuming the `Subscriber` and creating an [`InactiveSubscriber`]
    ///
    /// It might take up to 50 milliseconds until `RouDi` runs its discovery loop and the subscriber
    /// will be unsubscribed from the publisher.
    pub fn unsubscribe(self, sample_receiver: SampleReceiver<T, S>) -> InactiveSubscriber<T> {
        self.ffi_sub.as_ref().unsubscribe();

        drop(sample_receiver);

        InactiveSubscriber::from_ffi(self.ffi_sub.take())
    }
}
