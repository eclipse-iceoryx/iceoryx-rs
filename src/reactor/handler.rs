// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: © Contributors to the iceoryx-rs project
// SPDX-FileContributor: Mathias Kraus

//! Event handler dispatched by the reactor

use super::control::Foo;
use super::control::Condition;

pub trait Event {}

pub trait State {}

pub struct Handler {
    pub(super) target: Box<dyn Foo>,
    pub(super) condition: Box<dyn Fn(&dyn Foo) -> Condition>,
    pub(super) action: Box<dyn FnMut(&mut dyn Foo)>,
}

impl Handler {
    pub (crate) fn new(target: Box<dyn Foo>,
    condition: Box<dyn Fn(&dyn Foo) -> Condition>,
    action: Box<dyn FnMut(&mut dyn Foo)>) -> Self

    {
        Self {target, condition, action}
    }
}
