// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: © Contributors to the iceoryx-rs project
// SPDX-FileContributor: Mathias Kraus
// SPDX-FileContributor: Apex.AI

use crate::SubscriberOptions;

use std::ffi::{c_void, CString};
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
    #include "iceoryx_posh/internal/popo/building_blocks/condition_variable_data.hpp"
    #include "iceoryx_posh/runtime/posh_runtime.hpp"

    using iox::SubscribeState;
    using iox::capro::IdString_t;
    using iox::cxx::TruncateToCapacity;
    using iox::popo::QueueFullPolicy;
    using iox::popo::SubscriberOptions;
    using iox::popo::SubscriberPortUser;
    using iox::runtime::PoshRuntime;

    class ConditionVariable {
      public:
        ConditionVariable()
          : m_data(*PoshRuntime::getInstance().getMiddlewareConditionVariable())
        {}

        ~ConditionVariable() {
            m_data.m_toBeDestroyed.store(true, std::memory_order_relaxed);
            m_data.m_semaphore.post().or_else([](auto) {
                iox::LogFatal() << "Could not get ConditionVariableData from RouDi! Terminating!";
                std::terminate();
            });
        }

        void timedWait(const iox::units::Duration& timeToWait) {
            m_data.m_semaphore.timedWait(timeToWait).or_else([](auto) {
                iox::LogFatal() << "Could not wait on semaphore! Potentially corrupted! Terminating!";
                std::terminate();
            }).value();
        }

        iox::popo::ConditionVariableData& data() {
            return m_data;
        }

      private:
        iox::popo::ConditionVariableData& m_data;
    };
}}

cpp_class!(pub unsafe struct Subscriber as "SubscriberPortUser");
cpp_class!(pub unsafe struct ConditionVariable as "ConditionVariable");

impl Subscriber {
    pub fn new(
        service: &str,
        instance: &str,
        event: &str,
        options: &SubscriberOptions,
    ) -> Option<Box<Self>> {
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
        let queue_full_policy = options.queue_full_policy as u8;
        let requires_publisher_history_support = options.requires_publisher_history_support;
        unsafe {
            let raw = cpp!([service as "const char *",
                            instance as "const char *",
                            event as "const char *",
                            queue_capacity as "uint64_t",
                            history_request as "uint64_t",
                            node_name as "const char *",
                            subscribe_on_create as "bool",
                            queue_full_policy as "uint8_t",
                            requires_publisher_history_support as "bool"]
                            -> *mut Subscriber as "SubscriberPortUser*"
            {
                SubscriberOptions options;
                options.queueCapacity = queue_capacity;
                options.historyRequest = history_request;
                options.nodeName = IdString_t(TruncateToCapacity, node_name);
                options.subscribeOnCreate = subscribe_on_create;
                options.queueFullPolicy = static_cast<QueueFullPolicy>(queue_full_policy);
                options.requiresPublisherHistorySupport = requires_publisher_history_support;
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

            if raw.is_null() {
                None
            } else {
                Some(Box::from_raw(raw))
            }
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

    pub fn set_condition_variable(&self, condition_variable: &ConditionVariable) {
        unsafe {
            cpp!([self as "SubscriberPortUser*", condition_variable as "ConditionVariable*"] {
                if(!self->isConditionVariableSet()) {
                    // currently the condition variable is used only for one subscriber and therefore the index is set to 0
                    constexpr uint64_t NOTIFICATION_INDEX{0};
                    self->setConditionVariable(condition_variable->data(), NOTIFICATION_INDEX);
                }
            });
        }
    }

    pub fn unset_condition_variable(&self) {
        unsafe {
            cpp!([self as "SubscriberPortUser*"] {
                self->unsetConditionVariable();
            });
        }
    }

    pub fn is_condition_variable_set(&self) -> bool {
        unsafe {
            cpp!([self as "SubscriberPortUser*"] -> bool as "bool"{
                return self->isConditionVariableSet();
            })
        }
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

    pub fn get_chunk<T>(&self) -> Option<*const T> {
        unsafe {
            let payload = cpp!([self as "SubscriberPortUser*"] -> *const std::ffi::c_void as "const void*" {
                auto getChunkResult = self->tryGetChunk();

                if (getChunkResult.has_error()) {
                    return nullptr;
                }

                return getChunkResult.value()->userPayload();
            });

            if !payload.is_null() {
                Some(payload as *const T)
            } else {
                None
            }
        }
    }

    pub fn get_user_payload_size<T>(&self, payload: *const T) -> u32 {
        unsafe {
            let payload = payload as *const c_void;
            cpp!([payload as "void*"] -> u32 as "uint32_t" {
                auto header = iox::mepoo::ChunkHeader::fromUserPayload(payload);
                return header->userPayloadSize();
            })
        }
    }

    pub fn release_chunk<T: ?Sized>(&self, payload: *const T) {
        unsafe {
            let payload = payload as *const c_void;
            cpp!([self as "SubscriberPortUser*", payload as "void*"] {
                auto header = iox::mepoo::ChunkHeader::fromUserPayload(payload);
                self->releaseChunk(header);
            });
        }
    }
}

impl fmt::Debug for Subscriber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Subscriber")
            .field("_opaque", &self._opaque)
            .finish()
    }
}

impl ConditionVariable {
    pub fn new() -> Box<Self> {
        unsafe {
            let raw = cpp!([] -> *mut ConditionVariable as "ConditionVariable*"
            {
                return new ConditionVariable;
            });

            Box::from_raw(raw)
        }
    }

    pub fn timed_wait(&self, timeout: Duration) {
        unsafe {
            let timeout_ns = timeout.as_nanos() as u64;
            cpp!([self as "ConditionVariable*", timeout_ns as "uint64_t"] {
                self->timedWait(iox::units::Duration::fromNanoseconds(timeout_ns));
            });
        }
    }
}
