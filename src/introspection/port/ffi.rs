// Copyright 2020 Mathias Kraus - All rights reserved
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// http://www.apache.org/licenses/LICENSE-2.0>. This file may not be
// copied, modified, or distributed except according to those terms.

use crate::sb::Topic;

use std::ffi::CStr;
use std::os::raw::c_char;

cpp! {{
    #include "iceoryx_posh/roudi/introspection_types.hpp"

    using iox::roudi::PortData;
    using iox::roudi::ReceiverPortData;
    using iox::roudi::SenderPortData;
    using iox::roudi::PortIntrospectionFieldTopic;
}}

// TODO: this should be moved somewhere else
#[derive(Default, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct ServiceDescription {
    pub service_id: String,
    pub instance_id: String,
    pub event_id: String,
}

#[repr(C)]
#[derive(Debug)]
struct PortData {
    // here the port data follows, but it's all a iox::cxx::stringg and therefore we cannot directly access it
}

#[repr(C)]
#[derive(Debug)]
pub struct ReceiverPortData {
    port_data: PortData,
}

#[repr(C)]
#[derive(Debug)]
pub struct SenderPortData {
    port_data: PortData,
}

fn process_name<Port>(port: &Port) -> Option<String> {
    unsafe {
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
        let name = cpp!([port as "const PortData*"] -> *const c_char as "const char*" {
            return port->m_caproEventMethodID.c_str();
        });
        CStr::from_ptr(name)
            .to_str()
            .map_or(None, |name| Some(name.to_string()))
    }
}

fn runnable_name<Port>(port: &Port) -> Option<String> {
    unsafe {
        let name = cpp!([port as "const PortData*"] -> *const c_char as "const char*" {
            return port->m_runnable.c_str();
        });
        CStr::from_ptr(name)
            .to_str()
            .map_or(None, |name| Some(name.to_string()))
    }
}

impl ReceiverPortData {
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

    pub fn runnable_name(&self) -> Option<String> {
        runnable_name(self)
    }

    pub fn corresponding_sernder_port_index(&self) -> i32 {
        unsafe {
            cpp!([self as "const ReceiverPortData*"] -> i32 as "int32_t" {
                return self->m_senderIndex;
            })
        }
    }
}

impl SenderPortData {
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

    pub fn runnable_name(&self) -> Option<String> {
        runnable_name(self)
    }

    pub fn internal_sender_port_id(&self) -> u64 {
        unsafe {
            cpp!([self as "const SenderPortData*"] -> u64 as "uint64_t" {
                return self->m_senderPortID;
            })
        }
    }
}

pub struct ReceiverPortIntrospectionContainer<'a> {
    parent: &'a PortIntrospectionTopic,
    index: usize,
}

pub struct SenderPortIntrospectionContainer<'a> {
    parent: &'a PortIntrospectionTopic,
    index: usize,
}

#[repr(C)]
#[derive(Debug)]
pub struct PortIntrospectionTopic {
    // here the receiver/sender port data follows, but it's in a iox::cxx::Vector container and therefore we cannot directly access it from rust
}

impl PortIntrospectionTopic {
    pub fn new() -> Topic<Self> {
        Topic::<Self>::new("Introspection", "RouDi_ID", "Port")
    }

    pub fn receiver_ports(&self) -> ReceiverPortIntrospectionContainer {
        ReceiverPortIntrospectionContainer {
            parent: &*self,
            index: 0,
        }
    }

    pub fn sender_ports(&self) -> SenderPortIntrospectionContainer {
        SenderPortIntrospectionContainer {
            parent: &*self,
            index: 0,
        }
    }

    pub fn receiver_port_count(&self) -> usize {
        unsafe {
            cpp!([self as "const PortIntrospectionFieldTopic*"] -> usize as "size_t" {
                 return self->m_receiverList.size();
            })
        }
    }

    pub fn sender_port_count(&self) -> usize {
        unsafe {
            cpp!([self as "const PortIntrospectionFieldTopic*"] -> usize as "size_t" {
                 return self->m_senderList.size();
            })
        }
    }

    pub fn get_receiver_port(&self, index: usize) -> Option<&ReceiverPortData> {
        unsafe {
            let port = cpp!([self as "const PortIntrospectionFieldTopic*", index as "size_t"] -> *const ReceiverPortData as "const ReceiverPortData*" {
                 if (index >= self->m_receiverList.size()) {
                    return nullptr;
                 }
                 return &self->m_receiverList[index];
            });

            if !port.is_null() {
                Some(&*port)
            } else {
                None
            }
        }
    }

    pub fn get_sender_port(&self, index: usize) -> Option<&SenderPortData> {
        unsafe {
            let port = cpp!([self as "const PortIntrospectionFieldTopic*", index as "size_t"] -> *const SenderPortData as "const SenderPortData*" {
                 if (index >= self->m_senderList.size()) {
                    return nullptr;
                 }
                 return &self->m_senderList[index];
            });

            if !port.is_null() {
                Some(&*port)
            } else {
                None
            }
        }
    }
}

impl<'a> Iterator for ReceiverPortIntrospectionContainer<'a> {
    type Item = &'a ReceiverPortData;

    fn next(&mut self) -> Option<Self::Item> {
        let port = self.parent.get_receiver_port(self.index);
        if port.is_some() {
            self.index += 1;
        }
        port
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let topic = self.parent;
        unsafe {
            let size = cpp!([topic as "const PortIntrospectionFieldTopic*"] -> usize as "size_t" {
                 return topic->m_receiverList.size();
            });

            (size, Some(size))
        }
    }
}

impl<'a> Iterator for SenderPortIntrospectionContainer<'a> {
    type Item = &'a SenderPortData;

    fn next(&mut self) -> Option<Self::Item> {
        let port = self.parent.get_sender_port(self.index);
        if port.is_some() {
            self.index += 1;
        }
        port
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let topic = self.parent;
        unsafe {
            let size = cpp!([topic as "const PortIntrospectionFieldTopic*"] -> usize as "size_t" {
                 return topic->m_senderList.size();
            });

            (size, Some(size))
        }
    }
}
