// Copyright 2021 Mathias Kraus - All rights reserved
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// http://www.apache.org/licenses/LICENSE-2.0>. This file may not be
// copied, modified, or distributed except according to those terms.

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
