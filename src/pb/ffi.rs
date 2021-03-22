// Copyright 2020 Mathias Kraus - All rights reserved
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// http://www.apache.org/licenses/LICENSE-2.0>. This file may not be
// copied, modified, or distributed except according to those terms.

use crate::IceOryxError;
use crate::pb::PublisherOptions;

use std::ffi::CString;

cpp! {{
    #include "iceoryx_posh/internal/popo/ports/publisher_port_user.hpp"
    #include "iceoryx_posh/runtime/posh_runtime.hpp"

    using iox::capro::IdString_t;
    using iox::cxx::TruncateToCapacity;
    using iox::popo::PublisherOptions;
    using iox::popo::PublisherPortUser;
    using iox::runtime::PoshRuntime;
}}

cpp_class!(pub unsafe struct Publisher as "PublisherPortUser");

impl Publisher {
    pub(super) fn new(
        service: &str,
        instance: &str,
        event: &str,
        options: &PublisherOptions,
    ) -> Option<Box<Self>> {
        let service = CString::new(service).expect("CString::new failed");
        let service = service.as_ptr();
        let instance = CString::new(instance).expect("CString::new failed");
        let instance = instance.as_ptr();
        let event = CString::new(event).expect("CString::new failed");
        let event = event.as_ptr();
        let history_capacity = options.history_capacity;
        let node_name = CString::new(&options.node_name as &str).expect("CString::new failed");
        let node_name = node_name.as_ptr();
        let offer_on_create = options.offer_on_create;
        unsafe {
            let raw = cpp!([service as "const char *",
                            instance as "const char *",
                            event as "const char *",
                            history_capacity as "uint64_t",
                            node_name as "const char *",
                            offer_on_create as "bool"]
                            -> *mut Publisher as "PublisherPortUser*"
            {
                PublisherOptions options;
                options.historyCapacity = history_capacity;
                options.nodeName = IdString_t(TruncateToCapacity, node_name);
                options.offerOnCreate = offer_on_create;
                auto portData = PoshRuntime::getInstance().getMiddlewarePublisher(
                    {
                        IdString_t(TruncateToCapacity, service),
                        IdString_t(TruncateToCapacity, instance),
                        IdString_t(TruncateToCapacity, event)
                    },
                    options
                );
                return new PublisherPortUser(portData);
            });

            if raw.is_null() {
                None
            } else {
                Some(Box::from_raw(raw))
            }
        }
    }

    pub fn offer(&self) {
        unsafe {
            cpp!([self as "PublisherPortUser*"] {
                self->offer();
            });
        }
    }

    pub fn stop_offer(&self) {
        unsafe {
            cpp!([self as "PublisherPortUser*"] {
                self->stopOffer();
            });
        }
    }

    pub fn is_offered(&self) -> bool {
        unsafe {
            return cpp!([self as "PublisherPortUser*"] -> bool as "bool" {
                return self->isOffered();
            });
        }
    }

    pub fn has_subscribers(&self) -> bool {
        unsafe {
            return cpp!([self as "PublisherPortUser*"] -> bool as "bool" {
                return self->hasSubscribers();
            });
        }
    }

    pub fn allocate_chunk<T>(&self) -> Result<Box<T>, IceOryxError> {
        let payload_size = std::mem::size_of::<T>() as u32;
        unsafe {
            let chunk = cpp!([self as "PublisherPortUser*", payload_size as "uint32_t"] -> *mut std::ffi::c_void as "void*" {
                auto allocResult = self->tryAllocateChunk(payload_size);
                if (allocResult.has_error()) {
                    return nullptr;
                } else {
                    return allocResult.value()->payload();
                }
            });

            if !chunk.is_null() {
                Ok(Box::from_raw(chunk as *mut T))
            } else {
                Err(IceOryxError::ChunkAllocationFailed)
            }
        }
    }

    pub fn free_chunk<T>(&self, chunk: Box<T>) {
        unsafe {
            let chunk = Box::into_raw(chunk);
            cpp!([self as "PublisherPortUser*", chunk as "void*"] {
                auto header = iox::mepoo::ChunkHeader::fromPayload(chunk);
                self->releaseChunk(header);
            });
        }
    }

    pub fn send_chunk<T>(&self, chunk: Box<T>) {
        unsafe {
            let chunk = Box::into_raw(chunk);
            cpp!([self as "PublisherPortUser*", chunk as "void*"] {
                auto header = iox::mepoo::ChunkHeader::fromPayload(chunk);
                self->sendChunk(header);
            });
        }
    }
}
