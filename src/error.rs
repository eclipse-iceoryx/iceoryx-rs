// Copyright 2020 Mathias Kraus - All rights reserved
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// http://www.apache.org/licenses/LICENSE-2.0>. This file may not be
// copied, modified, or distributed except according to those terms.

use std::error::Error;
use std::fmt;

#[derive(Debug)]
// TODO use »thiserror« when number of errors increases
pub enum IceOryxError {
    ChunkAllocationFailed,
}

impl fmt::Display for IceOryxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            IceOryxError::ChunkAllocationFailed => write!(f, "ChunkAllocationFailed"),
        }
    }
}

impl Error for IceOryxError {
    fn description(&self) -> &str {
        match *self {
            IceOryxError::ChunkAllocationFailed => "Could not allocate chunk for sample",
        }
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            IceOryxError::ChunkAllocationFailed => None,
        }
    }
}
