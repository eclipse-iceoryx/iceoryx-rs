// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: © Contributors to the iceoryx-rs project
// SPDX-FileContributor: Mathias Kraus

mod memory;
pub use memory::MemPoolIntrospectionTopic;

mod port;
pub use port::PortIntrospectionTopic;

mod process;
pub use process::ProcessIntrospectionTopic;

#[derive(Default, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct ServiceDescription {
    pub service_id: String,
    pub instance_id: String,
    pub event_id: String,
}
