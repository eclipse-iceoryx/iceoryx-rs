// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: © Contributors to the iceoryx-rs project
// SPDX-FileContributor: Mathias Kraus

use std::marker::PhantomData;

pub(super) struct PublisherOptions
{
    pub history_capacity: u64,
    pub node_name: String,
    pub offer_on_create: bool,
    _phantom: PhantomData<()>,
}

impl Default for PublisherOptions {
    fn default() -> Self {
        Self {history_capacity: 0, node_name: String::new(), offer_on_create: true, _phantom: PhantomData}
    }
}
