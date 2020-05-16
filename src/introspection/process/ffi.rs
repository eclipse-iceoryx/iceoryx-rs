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

    using iox::roudi::ProcessIntrospectionData;
    using iox::roudi::ProcessIntrospectionFieldTopic;
}}

#[repr(C)]
#[derive(Debug)]
pub struct ProcessIntrospectionData {
    pid: i32,
    // here the process name follows, but it's a iox::cxx::stringg and therefore we cannot directly access it

    // here the runnable names follow, but it's in a iox::cxx::Vector container and therefore we cannot directly access it from rust
}

impl ProcessIntrospectionData {
    pub fn pid(&self) -> i32 {
        return self.pid;
    }

    pub fn name(&self) -> Option<String> {
        unsafe {
            let name = cpp!([self as "const ProcessIntrospectionData*"] -> *const c_char as "const char*" {
                return self->m_name.c_str();
            });
            CStr::from_ptr(name)
                .to_str()
                .map_or(None, |name| Some(name.to_string()))
        }
    }

    pub fn runnable_count(&self) -> usize {
        unsafe {
            cpp!([self as "const ProcessIntrospectionData*"] -> usize as "size_t" {
                 return self->m_runnables.size();
            })
        }
    }
}

pub struct ProcessIntrospectionContainer<'a> {
    parent: &'a ProcessIntrospectionTopic,
    index: usize,
}

#[repr(C)]
#[derive(Debug)]
pub struct ProcessIntrospectionTopic {
    // here the process data follows, but it's in a iox::cxx::Vector container and therefore we cannot directly access it from rust
}

impl ProcessIntrospectionTopic {
    pub fn new() -> Topic<Self> {
        Topic::<Self>::new("Introspection", "RouDi_ID", "Process")
    }

    pub fn processes(&self) -> ProcessIntrospectionContainer {
        ProcessIntrospectionContainer {
            parent: &*self,
            index: 0,
        }
    }

    pub fn process_count(&self) -> usize {
        unsafe {
            cpp!([self as "const ProcessIntrospectionFieldTopic*"] -> usize as "size_t" {
                 return self->m_processList.size();
            })
        }
    }

    pub fn get_process(&self, index: usize) -> Option<&ProcessIntrospectionData> {
        unsafe {
            let process = cpp!([self as "const ProcessIntrospectionFieldTopic*", index as "size_t"] -> *const ProcessIntrospectionData as "const ProcessIntrospectionData*" {
                 if (index >= self->m_processList.size()) {
                    return nullptr;
                 }
                 return &self->m_processList[index];
            });

            if !process.is_null() {
                Some(&*process)
            } else {
                None
            }
        }
    }
}

impl<'a> Iterator for ProcessIntrospectionContainer<'a> {
    type Item = &'a ProcessIntrospectionData;

    fn next(&mut self) -> Option<Self::Item> {
        let process = self.parent.get_process(self.index);
        if process.is_some() {
            self.index += 1;
        }
        process
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let topic = self.parent;
        unsafe {
            let size = cpp!([topic as "const ProcessIntrospectionFieldTopic*"] -> usize as "size_t" {
                 return topic->m_processList.size();
            });

            (size, Some(size))
        }
    }
}
