// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: © Contributors to the iceoryx-rs project
// SPDX-FileContributor: Mathias Kraus

//! Dispatches the events according to the registered handler

use super::Demultiplexer;

use std::sync::Arc;

pub struct Dispatcher {
    demux: Demultiplexer,
    // TODO queue receiver
}

impl Dispatcher {
    pub(super) fn new(condition_variable: Arc<Box<ffi::ConditionVariable>>) -> Self {
        Self {
            demux: Demultiplexer::new(condition_variable),
        }
    }
}
