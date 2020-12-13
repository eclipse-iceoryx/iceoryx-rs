// Copyright 2020 Mathias Kraus - All rights reserved
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// http://www.apache.org/licenses/LICENSE-2.0>. This file may not be
// copied, modified, or distributed except according to those terms.

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
