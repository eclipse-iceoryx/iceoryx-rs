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

/// The runtime that is needed for each application to communicate with the `RouDi` daemon
pub struct Runtime {}

impl Runtime {
    /// With this associated function the application registers at `RouDi`
    ///
    /// The call to this function is required in order to create publisher and subscriber and must be done early
    /// in the application startup. There cannot be two application with the same `app_name` be registered
    /// simultaneously at `RouDi`.
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
