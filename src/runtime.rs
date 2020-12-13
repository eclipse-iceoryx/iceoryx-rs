// Copyright 2020 Mathias Kraus - All rights reserved
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// http://www.apache.org/licenses/LICENSE-2.0>. This file may not be
// copied, modified, or distributed except according to those terms.

use std::ffi::CString;

cpp! {{
    #include "iceoryx_posh/runtime/posh_runtime.hpp"

    using iox::ProcessName_t;
    using iox::cxx::TruncateToCapacity;
    using iox::runtime::PoshRuntime;
}}

pub struct Runtime {}

impl Runtime {
    pub fn init(app_name: &str) {
        let app_name = CString::new(app_name).expect("CString::new failed");
        let app_name = app_name.as_ptr();
        unsafe {
            cpp!([app_name as "const char *"] {
                PoshRuntime::initRuntime(ProcessName_t(TruncateToCapacity, app_name));
            });
        }
    }
}
