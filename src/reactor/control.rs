// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: © Contributors to the iceoryx-rs project
// SPDX-FileContributor: Mathias Kraus

//! Control to register/unregister handler and stop the reactor

use super::{Event, Handler, State};

use std::any::Any;
use std::marker::PhantomData;
use std::sync::Arc;

pub trait Foo {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn attach_condition_variable(&self, condition_variable: &ffi::ConditionVariable, notification_index: u64);
}

pub struct AttachToken <T> {
    notification_index: u64,
    pub handler: Option<Handler>, // TODO remove handler
    _phantom: PhantomData<T>,
}

pub enum Condition {
    Event(bool),    // will be evaluated once when triggered, right before the handler is called
    State(bool),    // will be evaluated multiple times, right before and after the handler is called
}

pub struct Control {
    condition_variable: Arc<Box<ffi::ConditionVariable>>,
    notification_index: u64,
    // TODO queue sender
}

impl Control {
    pub(super) fn new(condition_variable: Arc<Box<ffi::ConditionVariable>>) -> Self {
        Self { condition_variable, notification_index: 0 }
    }

    pub fn attach<T: Foo + 'static>(&mut self, target: T, condition: Box<dyn Fn(&T) -> Condition>, mut action: Box<dyn FnMut(&mut T)>) -> AttachToken<T> {
        let condition = move |t: &dyn Foo| {
            unsafe { condition(t.as_any().downcast_ref::<T>().unwrap_unchecked()) }
        };
        let action = move |t: &mut dyn Foo| {
            unsafe { action(t.as_any_mut().downcast_mut::<T>().unwrap_unchecked()); }
        };

        let notification_index = self.notification_index;
        self.notification_index += 1;
        target.attach_condition_variable(&self.condition_variable, notification_index);
        let handler = Handler::new(Box::new(target), Box::new(condition), Box::new(action));

        AttachToken {notification_index, handler: Some(handler), _phantom: PhantomData }
    }

    pub fn detach<T: Foo>(&mut self, token: AttachToken<T>) -> T {
        unimplemented!()
    }
}
