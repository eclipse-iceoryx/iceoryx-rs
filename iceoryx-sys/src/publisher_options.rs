// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: © Contributors to the iceoryx-rs project
// SPDX-FileContributor: Mathias Kraus

use crate::ConsumerTooSlowPolicy;

use std::marker::PhantomData;

pub struct PublisherOptions {
    pub history_capacity: u64,
    pub node_name: String,
    pub offer_on_create: bool,
    pub subscriber_too_slow_policy: ConsumerTooSlowPolicy,
    _phantom: PhantomData<()>,
}

impl Default for PublisherOptions {
    fn default() -> Self {
        Self {
            history_capacity: 0,
            node_name: String::new(),
            offer_on_create: true,
            subscriber_too_slow_policy: ConsumerTooSlowPolicy::DiscardOldestData,
            _phantom: PhantomData,
        }
    }
}
