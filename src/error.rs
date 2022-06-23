// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: © Contributors to the iceoryx-rs project
// SPDX-FileContributor: Mathias Kraus

use thiserror::Error;

#[derive(Error, Debug)]
pub enum IceoryxError {
    #[error("could not allocate chunk")]
    SampleAllocationFailed,
    #[error("could not create publisher")]
    PublisherCreationFailed,
    #[error("could not create subscriber")]
    SubscriberCreationFailed,
    #[error("number of allowed chunks to hold is exhausted")]
    TooManyChunksHoldInParallel,
}
