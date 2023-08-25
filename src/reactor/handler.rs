// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: © Contributors to the iceoryx-rs project
// SPDX-FileContributor: Mathias Kraus

//! Event handler dispatched by the reactor

use std::marker::PhantomData;

pub trait Event {}

pub trait State {}

pub struct Handler<T> {
    _phantom: PhantomData<T>,
}
