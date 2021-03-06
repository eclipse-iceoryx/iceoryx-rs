// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: © Contributors to the iceoryx-rs project
// SPDX-FileContributor: Mathias Kraus

use crate::introspection::MemPoolIntrospectionTopic;
use crate::IceoryxError;
use crate::{InactiveSubscriber, SubscriberBuilder};

use std::marker::PhantomData;

/// Introspection for used memory and shared memory segments
pub struct MemPoolIntrospection {
    phantom: PhantomData<()>,
}

impl MemPoolIntrospection {
    /// Creates a subscriber for the MemPool introspection
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Result<InactiveSubscriber<MemPoolIntrospectionTopic>, IceoryxError> {
        SubscriberBuilder::<MemPoolIntrospectionTopic>::new("Introspection", "RouDi_ID", "MemPool")
            .queue_capacity(1)
            .history_request(1)
            .create_without_subscribe()
    }
}
