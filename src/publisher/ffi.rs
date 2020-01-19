// Copyright 2020 Mathias Kraus - All rights reserved
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// http://www.apache.org/licenses/LICENSE-2.0>. This file may not be
// copied, modified, or distributed except according to those terms.

use crate::IceOryxError;

use std::ffi::CString;

cpp! {{
    #include "iceoryx_posh/popo/publisher.hpp"

    using iox::popo::Publisher;
}}

cpp_class!(pub unsafe struct Publisher as "Publisher");

impl Publisher {
    pub fn new(service: &str, instance: &str, event: &str) -> Box<Self> {
        let service = CString::new(service).expect("CString::new failed");
        let service = service.as_ptr();
        let instance = CString::new(instance).expect("CString::new failed");
        let instance = instance.as_ptr();
        let event = CString::new(event).expect("CString::new failed");
        let event = event.as_ptr();
        unsafe {
            let raw = cpp!([service as "const char *", instance as "const char *", event as "const char *"] -> *mut Publisher as "Publisher*" {
                return new Publisher({service, instance, event});
            });

            Box::from_raw(raw)
        }
    }

    pub fn enable_delivery_on_subscription(&self) {
        unsafe {
            cpp!([self as "Publisher*"] {
                self->enableDoDeliverOnSubscription();
            });
        };
    }

    pub fn offer(&self) {
        unsafe {
            cpp!([self as "Publisher*"] {
                self->offer();
            });
        }
    }

    pub fn stop_offer(&self) {
        unsafe {
            cpp!([self as "Publisher*"] {
                self->stopOffer();
            });
        }
    }

    pub fn has_subscribers(&self) -> bool {
        unsafe {
            return cpp!([self as "Publisher*"] -> bool as "bool" {
                return self->hasSubscribers();
            });
        }
    }

    pub fn allocate_chunk<T>(&self) -> Result<Box<T>, IceOryxError> {
        let payload_size = std::mem::size_of::<T>() as u32;
        unsafe {
            let chunk = cpp!([self as "Publisher*", payload_size as "uint32_t"] -> *mut std::ffi::c_void as "void*" {
                return self->allocateChunk(payload_size);
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
            cpp!([self as "Publisher*", chunk as "void*"] {
                self->freeChunk(chunk);
            });
        }
    }

    pub fn send_chunk<T>(&self, chunk: Box<T>) {
        unsafe {
            let chunk = Box::into_raw(chunk);
            cpp!([self as "Publisher*", chunk as "void*"] {
                self->sendChunk(chunk);
            });
        }
    }
}
