// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: © Contributors to the iceoryx-rs project
// SPDX-FileContributor: Mathias Kraus

//! Dispatches the events according to the registered handler

use super::Demultiplexer;
use super::Condition;
use super::Handler;

use std::sync::Arc;
use std::time::{Duration, SystemTime};

pub struct Dispatcher {
    demux: Demultiplexer,
    targets: [Option<Handler>; 20],
    number_of_handler: u64,
    // TODO queue receiver
}

impl Dispatcher {
    pub(super) fn new(condition_variable: Arc<Box<ffi::ConditionVariable>>) -> Self {
        Self {
            demux: Demultiplexer::new(condition_variable),
            targets: Default::default(),
            number_of_handler: 0,
        }
    }

    // TODO remove
    pub fn add_handler(&mut self, handler: Handler) {
        self.targets[self.number_of_handler as usize] = Some(handler);
        self.number_of_handler += 1;
    }

    pub fn next_with_timeout(&mut self, timeout: Duration) -> Option<u64> {
        let entry_time = SystemTime::now();
        while let Some(remaining_timeout) = {
            let elapsed = entry_time.elapsed().unwrap_or(timeout);
            timeout.checked_sub(elapsed)
        } {
            self.demux.condition_variable.timed_wait(remaining_timeout);
            for index in 0..self.number_of_handler {
                let handler = self.targets[index as usize].as_mut().unwrap();
                if let Condition::State(true) = (handler.condition)(&*handler.target) {
                    (handler.action)(&mut *handler.target);
                }
                if index == self.number_of_handler -1 { return Some(0) }
            }
        }

        None
    }

//     pub fn wait_for_samples(&self, timeout: Duration) -> SampleReceiverWaitState {
//         if !self.ffi_sub.as_ref().is_condition_variable_set() {
//             return SampleReceiverWaitState::Stopped;
//         }
//         if self.has_data() {
//             return SampleReceiverWaitState::SamplesAvailable;
//         }
//
//         let entry_time = SystemTime::now();
//         while let Some(remaining_timeout) = {
//             let elapsed = entry_time.elapsed().unwrap_or(timeout);
//             timeout.checked_sub(elapsed)
//         } {
//             // self.condition_variable.timed_wait(remaining_timeout);
//             // if self.has_data() {
//             //     return SampleReceiverWaitState::SamplesAvailable;
//             // }
//         }
//
//         if self.ffi_sub.as_ref().is_condition_variable_set() {
//             SampleReceiverWaitState::Timeout
//         } else {
//             SampleReceiverWaitState::Stopped
//         }
//     }
}
