// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: © Contributors to the iceoryx-rs project
// SPDX-FileContributor: Mathias Kraus

use thiserror::Error;

#[derive(Error, Debug)]
pub enum IceOryxError {
    #[error("could not alloce a chunk")]
    ChunkAllocationFailed,
    #[error("could not create a publisher topic")]
    PublisherTopicCreationFailed,
    #[error("number of allowed chunks to hold is exhausted")]
    TooManyChunksHoldInParallel,
}
