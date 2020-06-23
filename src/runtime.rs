// Copyright 2020 Mathias Kraus - All rights reserved
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// http://www.apache.org/licenses/LICENSE-2.0>. This file may not be
// copied, modified, or distributed except according to those terms.

use std::ffi::CString;

cpp! {{
    #include "iceoryx_posh/runtime/posh_runtime.hpp"

    using iox::runtime::PoshRuntime;
}}

// TODO reenable once the Runtime is again aligned to 8 bytes
// cpp_class!(pub unsafe struct Runtime as "PoshRuntime");

// at the moment the workaround with an empty struct works,
// since we do not access any method from PoshRuntime
pub struct Runtime {}

static RUNTIME: Runtime = Runtime {};

impl Runtime {
    pub fn get_instance(app_name: &str) -> &'static Self {
        let app_name = CString::new(app_name).expect("CString::new failed");
        let app_name = app_name.as_ptr();
        unsafe {
            // TODO re-enable once the Runtime is again aligned to 8 bytes
            // return cpp!([app_name as "const char *"] -> &Runtime as "PoshRuntime*" {
            //     return &PoshRuntime::getInstance(app_name);
            cpp!([app_name as "const char *"] {
                PoshRuntime::getInstance(app_name);
            });

            &RUNTIME
        }
    }
}
