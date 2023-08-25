// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: © Contributors to the iceoryx-rs project
// SPDX-FileContributor: Mathias Kraus

// #![warn(missing_docs)]

//! # iceoryx-rs
//!
//! Eclipse iceoryx is a true zero-copy, inter-process communication framework with the goal to boost
//! autonomous driving with their demand on high data throughput and low latency. With its bandwidth and speed,
//! iceoryx also fits well into other domains where low latency and transmitting large data structures
//! is a concern. If you would like to know more about Eclipse iceoryx you can take a look at the
//! `Getting started` section on [iceoryx.io](https://iceoryx.io) or the
//! [README.md](https://github.com/eclipse-iceoryx/iceoryx/blob/master/README.md) of the main project.
//!
//! The Rust bindings are a work in progress and currently support only the pub-sub messaging pattern.
//! Upcoming releases will close the gap and the goal is to have the Rust bindings as a first class citizen
//! in the iceoryx ecosystem.
//!
//! This project started with the goal to create an awesome looking introspection TUI in Rust and led to
//! [iceray](https://crates.io/crates/iceray). Check it out.
//!
//! # Limitations
//!
//! Currently, only a subset of Eclipse iceoryx v2.0 is supported and some features are missing.
//!
//! - [x] pub-sub messaging pattern
//! - [ ] user defined header for pub-sub data
//! - [ ] request-response messaging pattern
//! - [ ] `Listener` and `WaitSet`
//! - [ ] lookup of available services aka `ServiceDiscovery`
//! - [x] accessing introspection topics like memory usage and available publisher and subscriber

pub mod introspection;
pub mod marker;
pub mod reactor;

mod error;
pub use error::IceoryxError;

mod publisher;
pub use publisher::InactivePublisher;
pub use publisher::Publisher;
pub use publisher::PublisherBuilder;

mod subscriber;
pub use subscriber::InactiveSubscriber;
pub use subscriber::Subscriber;
pub use subscriber::SubscriberBuilder;

mod sample_mut;
pub use sample_mut::SampleMut;

mod sample;
pub use sample::Sample;
pub use sample::SampleReceiver;
pub use sample::SampleReceiverWaitState;

pub mod st {
    //! Single-threaded restricted subscriber

    use super::*;

    /// A [`Sample`](sample::Sample) from a single-threaded subscriber
    pub type Sample<T> = sample::Sample<T, ffi::SubscriberRc>;
    /// A [`SampleReceiver`](sample::SampleReceiver) from a single-threaded subscriber
    pub type SampleReceiver<T> = sample::SampleReceiver<T, ffi::SubscriberRc>;
    /// A single-threaded [`Subscriber`](subscriber::Subscriber)
    pub type Subscriber<T> = subscriber::Subscriber<T, ffi::SubscriberRc>;
}

pub mod mt {
    //! Multi-threaded capable subscriber

    use super::*;

    /// A [`Sample`](sample::Sample) from a multi-threaded subscriber
    pub type Sample<T> = sample::Sample<T, ffi::SubscriberArc>;
    /// A [`SampleReceiver`](sample::SampleReceiver) from a multi-threaded subscriber
    pub type SampleReceiver<T> = sample::SampleReceiver<T, ffi::SubscriberArc>;
    /// A multi-threaded [`Subscriber`](subscriber::Subscriber)
    pub type Subscriber<T> = subscriber::Subscriber<T, ffi::SubscriberArc>;
}

// re-exports from iceoryx-sys
pub use ffi::ConsumerTooSlowPolicy;
pub use ffi::QueueFullPolicy;
pub use ffi::Runtime;
pub use ffi::SubscribeState;

#[cfg(test)]
pub(crate) mod testing {
    #[cfg(test)]
    pub(crate) use ffi::RouDiEnvironment;
}

#[cfg(test)]
mod tests;
