// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: © Contributors to the iceoryx-rs project
// SPDX-FileContributor: Mathias Kraus

pub mod introspection;
pub mod marker;

mod error;
pub use error::IceoryxError;

mod publisher;
pub use publisher::InactivePublisher;
pub use publisher::Publisher;
pub use publisher::PublisherBuilder;

mod subscriber;
pub use subscriber::InactiveSubscriber;
pub use subscriber::SubscriberBuilder;

mod sample_mut;
pub use sample_mut::SampleMut;

mod sample;
pub use sample::SampleReceiverWaitState;

pub mod st {
    use super::*;

    pub type Sample<T> = sample::Sample<T, ffi::SubscriberRc>;
    pub type SampleReceiver<T> = sample::SampleReceiver<T, ffi::SubscriberRc>;
    pub type Subscriber<T> = subscriber::Subscriber<T, ffi::SubscriberRc>;
}

pub mod mt {
    use super::*;

    pub type Sample<T> = sample::Sample<T, ffi::SubscriberArc>;
    pub type SampleReceiver<T> = sample::SampleReceiver<T, ffi::SubscriberArc>;
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
