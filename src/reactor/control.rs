// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: © Contributors to the iceoryx-rs project
// SPDX-FileContributor: Mathias Kraus

//! Control to register/unregister handler and stop the reactor

use super::{Event, Handler, State};

use std::sync::Arc;

pub struct Control {
    condition_variable: Arc<Box<ffi::ConditionVariable>>,
    // TODO queue sender
}

impl Control {
    pub(super) fn new(condition_variable: Arc<Box<ffi::ConditionVariable>>) -> Self {
        Self { condition_variable }
    }

    pub fn attach_event<T: Event>(source: Box<dyn Event>, handler: Box<Handler<T>>) {
        unimplemented!()
    }

    pub fn attach_state<T: State>(source: Box<dyn State>, handler: Box<Handler<T>>) {
        unimplemented!()
    }
}
