// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: © Contributors to the iceoryx-rs project
// SPDX-FileContributor: Mathias Kraus

use crate::{PublisherOptions, RawSampleMut};

use std::ffi::{c_void, CString};
use std::mem::MaybeUninit;

cpp! {{
    #include "iceoryx_posh/internal/popo/ports/publisher_port_user.hpp"
    #include "iceoryx_posh/runtime/posh_runtime.hpp"

    using iox::capro::IdString_t;
    using iox::cxx::TruncateToCapacity;
    using iox::popo::ConsumerTooSlowPolicy;
    using iox::popo::PublisherOptions;
    using iox::popo::PublisherPortUser;
    using iox::runtime::PoshRuntime;
}}

cpp_class!(pub unsafe struct Publisher as "PublisherPortUser");

impl Publisher {
    pub fn new(
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
        let subscriber_too_slow_policy = options.subscriber_too_slow_policy as u8;
        unsafe {
            let raw = cpp!([service as "const char *",
                            instance as "const char *",
                            event as "const char *",
                            history_capacity as "uint64_t",
                            node_name as "const char *",
                            offer_on_create as "bool",
                            subscriber_too_slow_policy as "uint8_t"]
                            -> *mut Publisher as "PublisherPortUser*"
            {
                PublisherOptions options;
                options.historyCapacity = history_capacity;
                options.nodeName = IdString_t(TruncateToCapacity, node_name);
                options.offerOnCreate = offer_on_create;
                options.subscriberTooSlowPolicy = static_cast<ConsumerTooSlowPolicy>(subscriber_too_slow_policy);
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

    pub fn try_allocate<T>(&self) -> Option<RawSampleMut<MaybeUninit<T>>> {
        let size = std::mem::size_of::<T>() as u32;
        let align = std::mem::align_of::<T>() as u32;
        unsafe {
            self.try_allocate_chunk(size, align)
                .map(|payload| payload.cast::<MaybeUninit<T>>())
        }
    }

    pub fn try_allocate_slice<T>(
        &self,
        len: u32,
        align: u32,
    ) -> Option<RawSampleMut<[MaybeUninit<T>]>> {
        unsafe {
            if align < std::mem::align_of::<T>() as u32 {
                return None;
            }

            let size = len * std::mem::size_of::<T>() as u32;
            self.try_allocate_chunk(size, align).map(|payload| {
                RawSampleMut::slice_from_raw_parts(payload.cast::<MaybeUninit<T>>(), len as usize)
            })
        }
    }

    unsafe fn try_allocate_chunk(&self, size: u32, align: u32) -> Option<RawSampleMut<c_void>> {
        let payload = cpp!([self as "PublisherPortUser*", size as "uint32_t", align as "uint32_t"] -> *mut std::ffi::c_void as "void*" {
            auto allocResult = self->tryAllocateChunk(size,
                                                      align,
                                                      iox::CHUNK_NO_USER_HEADER_SIZE,
                                                      iox::CHUNK_NO_USER_HEADER_ALIGNMENT);
            if (allocResult.has_error()) {
                return nullptr;
            } else {
                return allocResult.value()->userPayload();
            }
        });

        if !payload.is_null() {
            Some(RawSampleMut::new_unchecked(payload))
        } else {
            None
        }
    }

    pub fn release<T: ?Sized>(&self, sample: RawSampleMut<T>) {
        unsafe {
            let payload = sample.cast::<c_void>().as_payload_ptr();
            cpp!([self as "PublisherPortUser*", payload as "void*"] {
                auto header = iox::mepoo::ChunkHeader::fromUserPayload(payload);
                self->releaseChunk(header);
            });
        }
    }

    pub fn send<T: ?Sized>(&self, sample: RawSampleMut<T>) {
        let payload = sample.cast::<c_void>().as_payload_ptr();
        unsafe {
            cpp!([self as "PublisherPortUser*", payload as "void*"] {
                auto header = iox::mepoo::ChunkHeader::fromUserPayload(payload);
                self->sendChunk(header);
            });
        }
    }
}
