// Copyright 2020 Mathias Kraus - All rights reserved
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// http://www.apache.org/licenses/LICENSE-2.0>. This file may not be
// copied, modified, or distributed except according to those terms.

use crate::sb::Topic;

use std::ffi::CStr;
use std::marker::PhantomData;

cpp! {{
    #include "iceoryx_posh/roudi/introspection_types.hpp"

    using iox::roudi::MemPoolInfo;
    using iox::roudi::MemPoolIntrospectionTopic;
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
    parent: &'a MemPoolIntrospectionTopic,
    index: usize,
}

const GROUP_NAME_LENGTH: usize = 32;

#[repr(C)]
#[derive(Debug)]
pub struct MemPoolIntrospectionTopic {
    segment_id: u32,
    writer_group_name: [u8; GROUP_NAME_LENGTH],
    reader_group_name: [u8; GROUP_NAME_LENGTH],
    // here the mempool_info follows, but it's in a cxx::Vector container and therefore we cannot directly access it from rust
}

impl MemPoolIntrospectionTopic {
    pub fn new() -> Topic<Self> {
        Topic::<Self>::new("Introspection", "RouDi_ID", "MemPool")
    }
    pub fn segment_id(&self) -> u32 {
        self.segment_id
    }

    fn group_name_to_string(group_name: &[u8; GROUP_NAME_LENGTH]) -> Option<String> {
        let null_position = group_name.iter().position(|&c| c == 0)?;

        Some(
            CStr::from_bytes_with_nul(&group_name[0..=null_position])
                .ok()?
                .to_str()
                .ok()?
                .to_string(),
        )
    }

    pub fn writer_group(&self) -> Option<String> {
        Self::group_name_to_string(&self.writer_group_name)
    }
    pub fn reader_group(&self) -> Option<String> {
        Self::group_name_to_string(&self.reader_group_name)
    }
    pub fn mempools(&self) -> MemPoolInfoContainer {
        MemPoolInfoContainer {
            parent: &*self,
            index: 0,
        }
    }
}

impl<'a> Iterator for MemPoolInfoContainer<'a> {
    type Item = &'a MemPoolInfo;
    fn next(&mut self) -> Option<Self::Item> {
        let topic = self.parent;
        let index = self.index;
        unsafe {
            let mempool_info = cpp!([topic as "const MemPoolIntrospectionTopic*", index as "size_t"] -> *const MemPoolInfo as "const MemPoolInfo*" {
                 if (index >= topic->m_mempoolInfo.size()) {
                    return nullptr;
                 }
                 return &topic->m_mempoolInfo[index];
            });

            if !mempool_info.is_null() {
                self.index += 1;
                Some(&*mempool_info)
            } else {
                None
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let topic = self.parent;
        unsafe {
            let size = cpp!([topic as "const MemPoolIntrospectionTopic*"] -> usize as "size_t" {
                 return topic->m_mempoolInfo.size();
            });

            (size, Some(size))
        }
    }
}
