// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: © Contributors to the iceoryx-rs project
// SPDX-FileContributor: Mathias Kraus

#![recursion_limit = "256"]

#[macro_use]
extern crate cpp;

mod error;
mod queue_policy;
mod runtime;

pub mod introspection;
pub mod marker;
pub mod pb;
pub mod sb;

// re-export types
pub use error::IceoryxError;
pub use queue_policy::{ConsumerTooSlowPolicy, QueueFullPolicy};
pub use runtime::Runtime;

#[cfg(test)]
mod testing;
#[cfg(test)]
mod tests;
