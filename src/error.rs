// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: © Contributors to the iceoryx-rs project
// SPDX-FileContributor: Mathias Kraus

use thiserror::Error;

/// Error which can occur when using iceoryx
#[derive(Error, Debug)]
pub enum IceoryxError {
    /// Loaning a sample failed, e.g. due to exhausted memory pools.
    #[error("Could not loan sample")]
    LoanSampleFailed,
    /// The requested alignment is invalid, e.g. smaller than required by the underlying type
    #[error("Invalid alignment! Requested: {requested}; Min required: {min_required} ")]
    InvalidAlignment {
        /// The requested alignment
        requested: usize,
        /// The required minimal alignment
        min_required: usize,
    },
    /// Creation of the publisher failed, e.g. due to exhausted resources
    #[error("Could not create publisher")]
    PublisherCreationFailed,
    /// Creation of the subscriber failed, e.g. due to exhausted resources
    #[error("Could not create subscriber")]
    SubscriberCreationFailed,
    /// The number of maximum number of samples hold in parallel is exhausted
    #[error("Number of allowed samples to hold is exhausted")]
    TooManySamplesHoldInParallel,
}
