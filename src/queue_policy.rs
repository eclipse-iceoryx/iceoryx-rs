// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: © Contributors to the iceoryx-rs project
// SPDX-FileContributor: Mathias Kraus

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConsumerTooSlowPolicy {
    WaitForConsumer,
    DiscardOldestData,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueueFullPolicy {
    BlockProducer,
    DiscardOldestData,
}
