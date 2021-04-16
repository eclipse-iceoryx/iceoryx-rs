// SPDX-License-Identifier: Apache-2.0

use std::ffi::CString;

cpp! {{
    #include "iceoryx_posh/runtime/posh_runtime.hpp"

    using iox::RuntimeName_t;
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
                PoshRuntime::initRuntime(RuntimeName_t(TruncateToCapacity, app_name));
            });
        }
    }
}
