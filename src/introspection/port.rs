// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: © Contributors to the iceoryx-rs project
// SPDX-FileContributor: Mathias Kraus

use crate::introspection::PortIntrospectionTopic;
use crate::IceoryxError;
use crate::{InactiveSubscriber, SubscriberBuilder};

use std::marker::PhantomData;

pub struct PortIntrospection {
    phantom: PhantomData<()>,
}

impl PortIntrospection {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Result<InactiveSubscriber<PortIntrospectionTopic>, IceoryxError> {
        SubscriberBuilder::<PortIntrospectionTopic>::new("Introspection", "RouDi_ID", "Port")
            .queue_capacity(1)
            .history_request(1)
            .create_without_subscribe()
    }
}
