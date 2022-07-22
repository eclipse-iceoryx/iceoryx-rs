// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: © Contributors to the iceoryx-rs project
// SPDX-FileContributor: Mathias Kraus

#![recursion_limit = "256"]

#[macro_use]
extern crate cpp;

pub mod introspection;

mod chunk_header;
pub use chunk_header::ChunkHeader;

mod publisher;
pub use publisher::Publisher;

mod publisher_options;
pub use publisher_options::PublisherOptions;

mod queue_policy;
pub use queue_policy::ConsumerTooSlowPolicy;
pub use queue_policy::QueueFullPolicy;

mod runtime;
pub use runtime::Runtime;

mod subscriber;
pub use subscriber::ConditionVariable;
pub use subscriber::SubscribeState;
pub use subscriber::Subscriber;
pub use subscriber::SubscriberArc;
pub use subscriber::SubscriberRc;
pub use subscriber::SubscriberStrongRef;
pub use subscriber::SubscriberWeakRef;

mod subscriber_options;
pub use subscriber_options::SubscriberOptions;

// TODO add `testing` feature flag
mod roudi_environment;
pub use roudi_environment::RouDiEnvironment;
