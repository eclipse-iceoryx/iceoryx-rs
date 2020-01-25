// Copyright 2020 Mathias Kraus - All rights reserved
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// http://www.apache.org/licenses/LICENSE-2.0>. This file may not be
// copied, modified, or distributed except according to those terms.

use std::ffi::CString;
use std::fmt;
use std::time::Duration;

use std::rc::Rc;
use std::sync::Arc;

#[repr(C)]
#[derive(Debug, PartialEq)]
pub enum SubscriptionState {
    Subscribed,
    Unsubscribed,
    SubscriptionPending,
}

pub trait SubscriberStrongRef : Clone {
    fn new(ffi_sub: Box<Subscriber>) -> Self;

    fn as_ref(&self) -> &Subscriber;

    fn take(self) -> Box<Subscriber>;
}

pub trait SubscriberWeakRef {
    fn as_ref(&self) -> Option<&Subscriber>;
}

pub type SubscriberRc = Rc<Box<Subscriber>>;
// pub type SubscriberWeakRc = std::rc::Weak<Box<Subscriber>>;

pub type SubscriberArc = Arc<Box<Subscriber>>;
// pub type SubscriberWeakArc = std::sync::Weak<Box<Subscriber>>;


impl SubscriberStrongRef for SubscriberRc {
    fn new (ffi_sub: Box<Subscriber>) -> Self {
        Rc::new(ffi_sub)
    }

    fn as_ref(&self) -> &Subscriber {
        &*self
    }

    fn take (self) -> Box<Subscriber> {
        Rc::try_unwrap(self).expect("Unique owner of subscriber")
    }
}

impl SubscriberStrongRef for SubscriberArc {
    fn new (ffi_sub: Box<Subscriber>) -> Self {
        Arc::new(ffi_sub)
    }

    fn as_ref(&self) -> &Subscriber {
        &*self
    }

    fn take (self) -> Box<Subscriber> {
        Arc::try_unwrap(self).expect("Unique owner of subscriber")
    }
}

//TODO impl SubscriberWeakRef for ...

cpp! {{
    #include "iceoryx_posh/popo/subscriber.hpp"

    using iox::popo::Subscriber;
    using iox::popo::SubscriptionState;
}}

cpp_class!(pub unsafe struct Subscriber as "Subscriber");

impl Subscriber {
    pub fn new(service: &str, instance: &str, event: &str) -> Box<Self> {
        let service = CString::new(service).expect("CString::new failed");
        let service = service.as_ptr();
        let instance = CString::new(instance).expect("CString::new failed");
        let instance = instance.as_ptr();
        let event = CString::new(event).expect("CString::new failed");
        let event = event.as_ptr();
        unsafe {
            let raw = cpp!([service as "const char *", instance as "const char *", event as "const char *"] -> *mut Subscriber as "Subscriber*" {
                return new Subscriber(iox::capro::ServiceDescription(service, instance, event), "");
            });

            Box::from_raw(raw)
        }
    }

    pub fn subscribe(&self, cache_size: u32) {
        unsafe {
            cpp!([self as "Subscriber*", cache_size as "uint32_t"] {
                self->subscribe(cache_size);
            });
        }
    }

    pub fn unsubscribe(&self) {
        unsafe {
            cpp!([self as "Subscriber*"] {
                self->unsubscribe();
            });
        }
    }

    pub fn subscription_state(&self) -> SubscriptionState {
        unsafe {
            cpp!([self as "Subscriber*"] -> SubscriptionState as "SubscriptionState" {
                return self->getSubscriptionState();
            })
        }
    }

    pub fn enable_wait_for_chunks(&self) {
        unsafe {
            cpp!([self as "Subscriber*"] {
                if(!self->isChunkReceiveSemaphoreSet()) {
                    self->setChunkReceiveSemaphore(self->getSemaphore());
                }
            });
        }
    }

    pub fn wait_for_chunks_enabled(&self) -> bool {
        unsafe {
            cpp!([self as "Subscriber*"] -> bool as "bool"{
                return self->isChunkReceiveSemaphoreSet();
            })
        }
    }

    // TODO additional API in iceoryx needed
    // pub fn disable_wait_for_chunks(&self) {
    //     unsafe {
    //         cpp!([self as "Subscriber*"] {
    //             self->unsetChunkReceiveSemaphore();
    //         });
    //     }
    // }

    pub fn wait_for_chunks(&self, timeout: Duration) -> bool {
        let timeout = timeout.as_millis() as u32;
        unsafe {
            cpp!([self as "Subscriber*", timeout as "uint32_t"] -> bool as "bool" {
                return self->waitForChunk(timeout);
            })
        }
    }

    pub fn has_chunks(&self) -> bool {
        unsafe {
            cpp!([self as "Subscriber*"] -> bool as "bool" {
                return self->hasNewChunks();
            })
        }
    }

    pub fn clear(&self) {
        unsafe {
            cpp!([self as "Subscriber*"] {
                self->deleteNewChunks();
            });
        }
    }

    pub fn get_chunk<T>(&self) -> Option<Box<T>> {
        unsafe {
            let chunk = cpp!([self as "Subscriber*"] -> *const std::ffi::c_void as "const void*" {
                const void* chunk;
                auto ret = self->getChunk(&chunk);

                return ret ? chunk : nullptr;
            });

            if !chunk.is_null() {
                Some(Box::from_raw(chunk as *mut T))
            } else {
                None
            }
        }
    }

    pub fn release_chunk<T>(&self, chunk: Box<T>) {
        unsafe {
            let chunk = Box::into_raw(chunk);
            let mut chunk = &*chunk;
            cpp!([self as "Subscriber*", mut chunk as "void*"] {
                self->releaseChunk(chunk);
            });
        }
    }
}

impl fmt::Debug for Subscriber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}
