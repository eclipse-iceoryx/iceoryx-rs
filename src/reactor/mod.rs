// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: © Contributors to the iceoryx-rs project
// SPDX-FileContributor: Mathias Kraus

//! Implements the reactor pattern
//!
//! Subscriber can use the `Control` to register at the reactor and the `Dispatcher`
//! calls the registered handler functions.

use std::sync::Arc;

mod control;
pub use control::Control;
pub use control::Condition;
pub use control::Foo;

mod demultiplexer;
use demultiplexer::Demultiplexer;

mod dispatcher;
pub use dispatcher::Dispatcher;

mod handler;
pub use handler::Event;
pub use handler::Handler;
pub use handler::State;

pub struct Reactor {
    control: Control,
    dispatcher: Dispatcher,
}

impl Reactor {
    pub fn new() -> Self {
        let condition_variable = Arc::new(ffi::ConditionVariable::new());
        let control = Control::new(condition_variable.clone());
        let dispatcher = Dispatcher::new(condition_variable);

        Self {
            control,
            dispatcher,
        }
    }

    pub fn split(self) -> (Control, Dispatcher) {
        let Reactor { control, dispatcher } = self;
        (control, dispatcher)
    }
}
