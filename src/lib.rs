// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: © Contributors to the iceoryx-rs project
// SPDX-FileContributor: Mathias Kraus

mod error;

pub mod introspection;
pub mod marker;
pub mod pb;
pub mod sb;

// re-export types
pub use error::IceoryxError;
pub use pb::Publisher;
pub use pb::SampleMut;

pub use sb::InactiveSubscriber;
pub use sb::SubscriberBuilder;
pub use sb::{mt, st};

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
