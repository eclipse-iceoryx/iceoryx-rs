// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: © Contributors to the iceoryx-rs project
// SPDX-FileContributor: Mathias Kraus

use std::sync::Arc;

pub(super) struct Demultiplexer {
    pub(super) condition_variable: Arc<Box<ffi::ConditionVariable>>,
    // TODO queue receiver
}

impl Demultiplexer {
    pub(super) fn new(condition_variable: Arc<Box<ffi::ConditionVariable>>) -> Self {
        Self { condition_variable }
    }
}
