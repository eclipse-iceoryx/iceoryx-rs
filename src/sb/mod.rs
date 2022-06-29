// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: © Contributors to the iceoryx-rs project
// SPDX-FileContributor: Mathias Kraus

mod sample;
mod subscriber;

pub use sample::Sample;
pub use sample::SampleReceiverWaitState;
pub use subscriber::{InactiveSubscriber, SubscriberBuilder};

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
