// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: © Contributors to the iceoryx-rs project
// SPDX-FileContributor: Mathias Kraus

use crate::introspection::ProcessIntrospectionTopic;
use crate::IceoryxError;
use crate::{InactiveSubscriber, SubscriberBuilder};

use std::marker::PhantomData;

/// Introspection for registered processes
pub struct ProcessIntrospection {
    phantom: PhantomData<()>,
}

impl ProcessIntrospection {
    /// Creates a subscriber for the process introspection
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Result<InactiveSubscriber<ProcessIntrospectionTopic>, IceoryxError> {
        SubscriberBuilder::<ProcessIntrospectionTopic>::new("Introspection", "RouDi_ID", "Process")
            .queue_capacity(1)
            .history_request(1)
            .create_without_subscribe()
    }
}
