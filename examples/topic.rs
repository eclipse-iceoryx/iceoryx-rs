// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: © Contributors to the iceoryx-rs project
// SPDX-FileContributor: Mathias Kraus

use iceoryx_rs::marker::ShmSend;

#[repr(C)]
#[derive(Default)]
pub struct Counter {
    pub counter: u32,
}

unsafe impl ShmSend for Counter {}
