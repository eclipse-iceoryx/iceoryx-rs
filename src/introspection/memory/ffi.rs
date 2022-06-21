// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: © Contributors to the iceoryx-rs project
// SPDX-FileContributor: Mathias Kraus

use crate::sb::{InactiveSubscriber, SubscriberBuilder};

use std::ffi::CStr;
use std::marker::PhantomData;
use std::os::raw::c_char;

cpp! {{
    #include "iceoryx_posh/roudi/introspection_types.hpp"

    using iox::roudi::MemPoolInfo;
    using iox::roudi::MemPoolInfoContainer;
    using iox::roudi::MemPoolIntrospectionInfo;
    using iox::roudi::MemPoolIntrospectionInfoContainer;
}}

#[repr(C)]
#[derive(Debug)]
pub struct MemPoolInfo {
    pub used_chunks: u32,
    pub min_free_chunks: u32,
    pub total_number_of_chunks: u32,
    pub chunk_size: u32,
    pub payload_size: u32,
    phantom: PhantomData<()>,
}

pub struct MemPoolInfoContainer<'a> {
    memory_segment: &'a MemorySegment,
    mempool_index: usize,
}

#[repr(C)]
#[derive(Debug)]
pub struct MemorySegment {
    segment_id: u32,
    // here the reader/writer group names follow; while they are fixed size c_char array,
    // we would have to manually keep the length in sync with the C++ part, therefore no direct access

    // here the mempool_info follows, but it's in a iox::cxx::Vector container and therefore we cannot directly access it from rust
}

impl MemorySegment {
    pub fn segment_id(&self) -> u32 {
        self.segment_id
    }

    pub fn writer_group(&self) -> Option<String> {
        unsafe {
            let group_name = cpp!([self as "const MemPoolIntrospectionInfo*"] -> *const c_char as "const char*" {
                return self->m_writerGroupName.c_str();
            });
            CStr::from_ptr(group_name)
                .to_str()
                .map_or(None, |group_name| Some(group_name.to_string()))
        }
    }

    pub fn reader_group(&self) -> Option<String> {
        unsafe {
            let group_name = cpp!([self as "const MemPoolIntrospectionInfo*"] -> *const c_char as "const char*" {
                return self->m_readerGroupName.c_str();
            });
            CStr::from_ptr(group_name)
                .to_str()
                .map_or(None, |group_name| Some(group_name.to_string()))
        }
    }

    pub fn mempools(&self) -> MemPoolInfoContainer {
        MemPoolInfoContainer {
            memory_segment: &*self,
            mempool_index: 0,
        }
    }
}

impl<'a> Iterator for MemPoolInfoContainer<'a> {
    type Item = &'a MemPoolInfo;

    fn next(&mut self) -> Option<Self::Item> {
        let memory_segment = self.memory_segment;
        let mempool_index = self.mempool_index;
        unsafe {
            let mempool_info = cpp!([memory_segment as "const MemPoolIntrospectionInfo*", mempool_index as "size_t"] -> *const MemPoolInfo as "const MemPoolInfo*" {
                 if (mempool_index >= memory_segment->m_mempoolInfo.size()) {
                    return nullptr;
                 }
                 return &memory_segment->m_mempoolInfo[mempool_index];
            });

            if !mempool_info.is_null() {
                self.mempool_index += 1;
                Some(&*mempool_info)
            } else {
                None
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let memory_segment = self.memory_segment;
        unsafe {
            let size = cpp!([memory_segment as "const MemPoolIntrospectionInfo*"] -> usize as "size_t" {
                 return memory_segment->m_mempoolInfo.size();
            });

            (size, Some(size))
        }
    }
}

pub struct MemorySegmentContainer<'a> {
    memory_segments: &'a MemPoolIntrospectionTopic,
    segment_index: usize,
}

pub struct MemPoolIntrospectionTopic {
    // this is actually the MemPoolIntrospectionInfoContainer with the memory segment introspection
}

impl MemPoolIntrospectionTopic {
    pub fn new() -> InactiveSubscriber<Self> {
        SubscriberBuilder::<Self>::new("Introspection", "RouDi_ID", "MemPool")
            .queue_capacity(1)
            .history_request(1)
            .create_without_subscribe()
            .expect("Create subscriber")
    }

    pub fn memory_segments(&self) -> MemorySegmentContainer {
        MemorySegmentContainer {
            memory_segments: &*self,
            segment_index: 0,
        }
    }
}

impl<'a> Iterator for MemorySegmentContainer<'a> {
    type Item = &'a MemorySegment;

    fn next(&mut self) -> Option<Self::Item> {
        let memory_segments = self.memory_segments;
        let segment_index = self.segment_index;
        unsafe {
            let segment = cpp!([memory_segments as "const MemPoolIntrospectionInfoContainer*", segment_index as "size_t"] -> *const MemorySegment as "const MemPoolIntrospectionInfo*" {
                 if (segment_index >= memory_segments->size()) {
                    return nullptr;
                 }
                 return &(*memory_segments)[segment_index];
            });

            if !segment.is_null() {
                self.segment_index += 1;
                Some(&*segment)
            } else {
                None
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let memory_segments = self.memory_segments;
        unsafe {
            let size = cpp!([memory_segments as "const MemPoolIntrospectionInfoContainer*"] -> usize as "size_t" {
                 return memory_segments->size();
            });

            (size, Some(size))
        }
    }
}
