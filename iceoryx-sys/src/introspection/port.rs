// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: © Contributors to the iceoryx-rs project
// SPDX-FileContributor: Mathias Kraus

use crate::introspection::ServiceDescription;

use std::ffi::CStr;
use std::marker::PhantomData;
use std::os::raw::c_char;

cpp! {{
    #include "iceoryx_posh/roudi/introspection_types.hpp"

    using iox::roudi::PortData;
    using iox::roudi::SubscriberPortData;
    using iox::roudi::PublisherPortData;
    using iox::roudi::PortIntrospectionFieldTopic;
}}

#[repr(C)]
#[derive(Debug)]
struct PortData {
    phantom: PhantomData<()>,
    // here the port data follows, but it's all a iox::cxx::stringg and therefore we cannot directly access it
}

#[repr(C)]
#[derive(Debug)]
pub struct SubscriberPortData {
    port_data: PortData,
}

#[repr(C)]
#[derive(Debug)]
pub struct PublisherPortData {
    port_data: PortData,
}

fn process_name<Port>(port: &Port) -> Option<String> {
    unsafe {
        let port = port as *const Port;
        let name = cpp!([port as "const PortData*"] -> *const c_char as "const char*" {
            return port->m_name.c_str();
        });
        CStr::from_ptr(name)
            .to_str()
            .map_or(None, |name| Some(name.to_string()))
    }
}

fn service_id<Port>(port: &Port) -> Option<String> {
    unsafe {
        let port = port as *const Port;
        let name = cpp!([port as "const PortData*"] -> *const c_char as "const char*" {
            return port->m_caproServiceID.c_str();
        });
        CStr::from_ptr(name)
            .to_str()
            .map_or(None, |name| Some(name.to_string()))
    }
}

fn instance_id<Port>(port: &Port) -> Option<String> {
    unsafe {
        let port = port as *const Port;
        let name = cpp!([port as "const PortData*"] -> *const c_char as "const char*" {
            return port->m_caproInstanceID.c_str();
        });
        CStr::from_ptr(name)
            .to_str()
            .map_or(None, |name| Some(name.to_string()))
    }
}

fn event_id<Port>(port: &Port) -> Option<String> {
    unsafe {
        let port = port as *const Port;
        let name = cpp!([port as "const PortData*"] -> *const c_char as "const char*" {
            return port->m_caproEventMethodID.c_str();
        });
        CStr::from_ptr(name)
            .to_str()
            .map_or(None, |name| Some(name.to_string()))
    }
}

fn node_name<Port>(port: &Port) -> Option<String> {
    unsafe {
        let port = port as *const Port;
        let name = cpp!([port as "const PortData*"] -> *const c_char as "const char*" {
            return port->m_node.c_str();
        });
        CStr::from_ptr(name)
            .to_str()
            .map_or(None, |name| Some(name.to_string()))
    }
}

impl SubscriberPortData {
    pub fn process_name(&self) -> Option<String> {
        process_name(self)
    }

    pub fn service_description(&self) -> Option<ServiceDescription> {
        match (service_id(self), instance_id(self), event_id(self)) {
            (Some(service_id), Some(instance_id), Some(event_id)) => Some(ServiceDescription {
                service_id,
                instance_id,
                event_id,
            }),
            _ => None,
        }
    }

    pub fn node_name(&self) -> Option<String> {
        node_name(self)
    }
}

impl PublisherPortData {
    pub fn process_name(&self) -> Option<String> {
        process_name(self)
    }

    pub fn service_description(&self) -> Option<ServiceDescription> {
        match (service_id(self), instance_id(self), event_id(self)) {
            (Some(service_id), Some(instance_id), Some(event_id)) => Some(ServiceDescription {
                service_id,
                instance_id,
                event_id,
            }),
            _ => None,
        }
    }

    pub fn node_name(&self) -> Option<String> {
        node_name(self)
    }

    pub fn internal_publisher_port_id(&self) -> u64 {
        unsafe {
            let this_ptr = self as *const Self;
            cpp!([this_ptr as "const PublisherPortData*"] -> u64 as "uint64_t" {
                return this_ptr->m_publisherPortID;
            })
        }
    }
}

pub struct SubscriberPortIntrospectionContainer<'a> {
    parent: &'a PortIntrospectionTopic,
    index: usize,
}

pub struct PublisherPortIntrospectionContainer<'a> {
    parent: &'a PortIntrospectionTopic,
    index: usize,
}

#[repr(C)]
#[derive(Debug)]
pub struct PortIntrospectionTopic {
    phantom: PhantomData<()>,
    // here the subscriber/publisher port data follows, but it's in a iox::cxx::Vector container and therefore we cannot directly access it from rust
}

impl PortIntrospectionTopic {
    pub fn subscriber_ports(&self) -> SubscriberPortIntrospectionContainer {
        SubscriberPortIntrospectionContainer {
            parent: self,
            index: 0,
        }
    }

    pub fn publisher_ports(&self) -> PublisherPortIntrospectionContainer {
        PublisherPortIntrospectionContainer {
            parent: self,
            index: 0,
        }
    }

    pub fn subscriber_port_count(&self) -> usize {
        unsafe {
            let this_ptr = self as *const Self;
            cpp!([this_ptr as "const PortIntrospectionFieldTopic*"] -> usize as "size_t" {
                 return this_ptr->m_subscriberList.size();
            })
        }
    }

    pub fn publisher_port_count(&self) -> usize {
        unsafe {
            let this_ptr = self as *const Self;
            cpp!([this_ptr as "const PortIntrospectionFieldTopic*"] -> usize as "size_t" {
                 return this_ptr->m_publisherList.size();
            })
        }
    }

    pub fn get_subscriber_port(&self, index: usize) -> Option<&SubscriberPortData> {
        unsafe {
            let this_ptr = self as *const Self;
            let port = cpp!([this_ptr as "const PortIntrospectionFieldTopic*", index as "size_t"] -> *const SubscriberPortData as "const SubscriberPortData*" {
                 if (index >= this_ptr->m_subscriberList.size()) {
                    return nullptr;
                 }
                 return &this_ptr->m_subscriberList[index];
            });

            if !port.is_null() {
                Some(&*port)
            } else {
                None
            }
        }
    }

    pub fn get_publisher_port(&self, index: usize) -> Option<&PublisherPortData> {
        unsafe {
            let this_ptr = self as *const Self;
            let port = cpp!([this_ptr as "const PortIntrospectionFieldTopic*", index as "size_t"] -> *const PublisherPortData as "const PublisherPortData*" {
                 if (index >= this_ptr->m_publisherList.size()) {
                    return nullptr;
                 }
                 return &this_ptr->m_publisherList[index];
            });

            if !port.is_null() {
                Some(&*port)
            } else {
                None
            }
        }
    }
}

impl<'a> Iterator for SubscriberPortIntrospectionContainer<'a> {
    type Item = &'a SubscriberPortData;

    fn next(&mut self) -> Option<Self::Item> {
        let port = self.parent.get_subscriber_port(self.index);
        if port.is_some() {
            self.index += 1;
        }
        port
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let topic = self.parent as *const PortIntrospectionTopic;
        unsafe {
            let size = cpp!([topic as "const PortIntrospectionFieldTopic*"] -> usize as "size_t" {
                 return topic->m_subscriberList.size();
            });

            (size, Some(size))
        }
    }
}

impl<'a> Iterator for PublisherPortIntrospectionContainer<'a> {
    type Item = &'a PublisherPortData;

    fn next(&mut self) -> Option<Self::Item> {
        let port = self.parent.get_publisher_port(self.index);
        if port.is_some() {
            self.index += 1;
        }
        port
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let topic = self.parent as *const PortIntrospectionTopic;
        unsafe {
            let size = cpp!([topic as "const PortIntrospectionFieldTopic*"] -> usize as "size_t" {
                 return topic->m_publisherList.size();
            });

            (size, Some(size))
        }
    }
}
