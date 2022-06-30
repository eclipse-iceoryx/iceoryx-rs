// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: © Contributors to the iceoryx-rs project
// SPDX-FileContributor: Mathias Kraus

use std::ffi::CString;

cpp! {{
    #include "iceoryx_hoofs/log/logmanager.hpp"
    #include "iceoryx_posh/runtime/posh_runtime.hpp"

    using iox::RuntimeName_t;
    using iox::cxx::TruncateToCapacity;
    using iox::log::LogManager;
    using iox::log::LogLevelOutput;
    using iox::runtime::PoshRuntime;
}}

pub struct Runtime {}

impl Runtime {
    pub fn init(app_name: &str) {
        let app_name = CString::new(app_name).expect("CString::new failed");
        let app_name = app_name.as_ptr();
        unsafe {
            cpp!([app_name as "const char *"] {
                iox::log::LogManager::GetLogManager().SetDefaultLogLevel(iox::log::LogLevel::kWarn, LogLevelOutput::kHideLogLevel);
                PoshRuntime::initRuntime(RuntimeName_t(TruncateToCapacity, app_name));
            });
        }
    }
}
