// Copyright 2021 Mathias Kraus - All rights reserved
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// http://www.apache.org/licenses/LICENSE-2.0>. This file may not be
// copied, modified, or distributed except according to those terms.

use std::marker::PhantomData;

pub struct SubscriberOptions
{
    pub queue_capacity: u64,
    pub history_request: u64,
    pub node_name: String,
    pub subscribe_on_create: bool,
    _phantom: PhantomData<()>,
}

impl Default for SubscriberOptions {
    fn default() -> Self {
        Self {queue_capacity: 256, history_request: 0, node_name: String::new(), subscribe_on_create: true, _phantom: PhantomData}
    }
}

impl SubscriberOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
