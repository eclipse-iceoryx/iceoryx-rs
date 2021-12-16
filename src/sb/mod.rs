// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: © Contributors to the iceoryx-rs project
// SPDX-FileContributor: Mathias Kraus

mod ffi;
mod sample;
mod subscriber;
mod subscriber_options;
mod topic;

pub use ffi::SubscribeState;
pub use sample::Sample;
pub use sample::SampleReceiverWaitState;
pub use topic::{Topic, TopicBuilder};

use subscriber_options::SubscriberOptions;

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
