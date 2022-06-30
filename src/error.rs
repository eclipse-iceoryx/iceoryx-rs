// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: © Contributors to the iceoryx-rs project
// SPDX-FileContributor: Mathias Kraus

use thiserror::Error;

#[derive(Error, Debug)]
pub enum IceoryxError {
    #[error("Could not loan sample")]
    LoanSampleFailed,
    #[error("Could not create publisher")]
    PublisherCreationFailed,
    #[error("Could not create subscriber")]
    SubscriberCreationFailed,
    #[error("Number of allowed samples to hold is exhausted")]
    TooManySamplesHoldInParallel,
}
