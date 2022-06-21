// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: © Contributors to the iceoryx-rs project
// SPDX-FileContributor: Mathias Kraus

use crate::QueueFullPolicy;

use std::marker::PhantomData;

pub(super) struct SubscriberOptions {
    pub queue_capacity: u64,
    pub history_request: u64,
    pub node_name: String,
    pub subscribe_on_create: bool,
    pub queue_full_policy: QueueFullPolicy,
    pub requires_publisher_history_support: bool,
    _phantom: PhantomData<()>,
}

impl Default for SubscriberOptions {
    fn default() -> Self {
        Self {
            queue_capacity: 256,
            history_request: 0,
            node_name: String::new(),
            subscribe_on_create: true,
            queue_full_policy: QueueFullPolicy::DiscardOldestData,
            requires_publisher_history_support: false,
            _phantom: PhantomData,
        }
    }
}
