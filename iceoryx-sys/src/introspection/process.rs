// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: © Contributors to the iceoryx-rs project
// SPDX-FileContributor: Mathias Kraus

use std::ffi::CStr;
use std::marker::PhantomData;
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
    // here the process name follows, but it's a iox::cxx::string and therefore we cannot directly access it

    // here the node names follow, but it's in a iox::cxx::Vector container and therefore we cannot directly access it from rust
}

impl ProcessIntrospectionData {
    pub fn pid(&self) -> i32 {
        self.pid
    }

    pub fn name(&self) -> Option<String> {
        unsafe {
            let this_ptr = self as *const Self;
            let name = cpp!([this_ptr as "const ProcessIntrospectionData*"] -> *const c_char as "const char*" {
                return this_ptr->m_name.c_str();
            });
            CStr::from_ptr(name)
                .to_str()
                .map_or(None, |name| Some(name.to_string()))
        }
    }

    pub fn node_count(&self) -> usize {
        unsafe {
            let this_ptr = self as *const Self;
            cpp!([this_ptr as "const ProcessIntrospectionData*"] -> usize as "size_t" {
                 return this_ptr->m_nodes.size();
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
    phantom: PhantomData<()>,
    // here the process data follows, but it's in a iox::cxx::Vector container and therefore we cannot directly access it from rust
}

impl ProcessIntrospectionTopic {
    pub fn processes(&self) -> ProcessIntrospectionContainer {
        ProcessIntrospectionContainer {
            parent: self,
            index: 0,
        }
    }

    pub fn process_count(&self) -> usize {
        unsafe {
            let this_ptr = self as *const Self;
            cpp!([this_ptr as "const ProcessIntrospectionFieldTopic*"] -> usize as "size_t" {
                 return this_ptr->m_processList.size();
            })
        }
    }

    pub fn get_process(&self, index: usize) -> Option<&ProcessIntrospectionData> {
        unsafe {
            let this_ptr = self as *const Self;
            let process = cpp!([this_ptr as "const ProcessIntrospectionFieldTopic*", index as "size_t"] -> *const ProcessIntrospectionData as "const ProcessIntrospectionData*" {
                 if (index >= this_ptr->m_processList.size()) {
                    return nullptr;
                 }
                 return &this_ptr->m_processList[index];
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
        let topic = self.parent as *const ProcessIntrospectionTopic;
        unsafe {
            let size = cpp!([topic as "const ProcessIntrospectionFieldTopic*"] -> usize as "size_t" {
                 return topic->m_processList.size();
            });

            (size, Some(size))
        }
    }
}
