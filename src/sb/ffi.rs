// SPDX-License-Identifier: Apache-2.0

use crate::sb::SubscriberOptions;

use std::ffi::CString;
use std::fmt;
use std::time::Duration;

use std::rc::Rc;
use std::sync::Arc;

#[repr(C)]
#[derive(Debug, PartialEq)]
pub enum SubscribeState {
    NotSubscribed,
    SubscribeRequested,
    Subscribed,
    UnsubscribeRequested,
    WaitForOffer,
}

pub trait SubscriberStrongRef: Clone {
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
    fn new(ffi_sub: Box<Subscriber>) -> Self {
        Rc::new(ffi_sub)
    }

    fn as_ref(&self) -> &Subscriber {
        &*self
    }

    fn take(self) -> Box<Subscriber> {
        Rc::try_unwrap(self).expect("Unique owner of subscriber")
    }
}

impl SubscriberStrongRef for SubscriberArc {
    fn new(ffi_sub: Box<Subscriber>) -> Self {
        Arc::new(ffi_sub)
    }

    fn as_ref(&self) -> &Subscriber {
        &*self
    }

    fn take(self) -> Box<Subscriber> {
        Arc::try_unwrap(self).expect("Unique owner of subscriber")
    }
}

//TODO impl SubscriberWeakRef for ...

cpp! {{
    #include "iceoryx_posh/internal/popo/ports/subscriber_port_user.hpp"
    #include "iceoryx_posh/runtime/posh_runtime.hpp"

    using iox::SubscribeState;
    using iox::capro::IdString_t;
    using iox::cxx::TruncateToCapacity;
    using iox::popo::SubscriberOptions;
    using iox::popo::SubscriberPortUser;
    using iox::runtime::PoshRuntime;
}}

cpp_class!(pub unsafe struct Subscriber as "SubscriberPortUser");

impl Subscriber {
    pub(super) fn new(
        service: &str,
        instance: &str,
        event: &str,
        options: &SubscriberOptions,
    ) -> Box<Self> {
        let service = CString::new(service).expect("CString::new failed");
        let service = service.as_ptr();
        let instance = CString::new(instance).expect("CString::new failed");
        let instance = instance.as_ptr();
        let event = CString::new(event).expect("CString::new failed");
        let event = event.as_ptr();
        let queue_capacity = options.queue_capacity;
        let history_request = options.history_request;
        let node_name = CString::new(&options.node_name as &str).expect("CString::new failed");
        let node_name = node_name.as_ptr();
        let subscribe_on_create = options.subscribe_on_create;
        unsafe {
            let raw = cpp!([service as "const char *",
                            instance as "const char *",
                            event as "const char *",
                            queue_capacity as "uint64_t",
                            history_request as "uint64_t",
                            node_name as "const char *",
                            subscribe_on_create as "bool"]
                            -> *mut Subscriber as "SubscriberPortUser*"
            {
                SubscriberOptions options;
                options.queueCapacity = queue_capacity;
                options.historyRequest = history_request;
                options.nodeName = IdString_t(TruncateToCapacity, node_name);
                options.subscribeOnCreate = subscribe_on_create;
                auto portData = PoshRuntime::getInstance().getMiddlewareSubscriber(
                    {
                        IdString_t(TruncateToCapacity, service),
                        IdString_t(TruncateToCapacity, instance),
                        IdString_t(TruncateToCapacity, event)
                    },
                    options
                );
                return new SubscriberPortUser(portData);
            });

            Box::from_raw(raw)
        }
    }

    pub fn subscribe(&self) {
        unsafe {
            cpp!([self as "SubscriberPortUser*"] {
                self->subscribe();
            });
        }
    }

    pub fn unsubscribe(&self) {
        unsafe {
            cpp!([self as "SubscriberPortUser*"] {
                self->unsubscribe();
            });
        }
    }

    pub fn subscription_state(&self) -> SubscribeState {
        unsafe {
            cpp!([self as "SubscriberPortUser*"] -> SubscribeState as "SubscribeState" {
                return self->getSubscriptionState();
            })
        }
    }

    pub fn enable_wait_for_chunks(&self) {
        // TODO adjust to new iceoryx API
        // unsafe {
        //     cpp!([self as "SubscriberPortUser*"] {
        //         if(!self->isChunkReceiveSemaphoreSet()) {
        //             self->setChunkReceiveSemaphore(self->getSemaphore());
        //         }
        //     });
        // }
    }

    pub fn wait_for_chunks_enabled(&self) -> bool {
        // TODO adjust to new iceoryx API
        // unsafe {
        //     cpp!([self as "SubscriberPortUser*"] -> bool as "bool"{
        //         return self->isChunkReceiveSemaphoreSet();
        //     })
        // }
        false
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
        // TODO adjust to new iceoryx API
        // let timeout = timeout.as_millis() as u32;
        // unsafe {
        //     cpp!([self as "SubscriberPortUser*", timeout as "uint32_t"] -> bool as "bool" {
        //         return self->waitForChunk(timeout);
        //     })
        // }
        false
    }

    pub fn has_chunks(&self) -> bool {
        unsafe {
            cpp!([self as "SubscriberPortUser*"] -> bool as "bool" {
                return self->hasNewChunks();
            })
        }
    }

    // TODO has_chunks_lost_since_last_call

    pub fn clear(&self) {
        unsafe {
            cpp!([self as "SubscriberPortUser*"] {
                self->releaseQueuedChunks();
            });
        }
    }

    pub fn get_chunk<T>(&self) -> Option<Box<T>> {
        unsafe {
            let chunk = cpp!([self as "SubscriberPortUser*"] -> *const std::ffi::c_void as "const void*" {
                auto getChunkResult = self->tryGetChunk();

                if (getChunkResult.has_error()) {
                    return nullptr;
                }

                return getChunkResult.value()->userPayload();
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
            cpp!([self as "SubscriberPortUser*", mut chunk as "void*"] {
                auto header = iox::mepoo::ChunkHeader::fromUserPayload(chunk);
                self->releaseChunk(header);
            });
        }
    }
}

impl fmt::Debug for Subscriber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}
