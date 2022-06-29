// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: © Contributors to the iceoryx-rs project
// SPDX-FileContributor: Mathias Kraus

cpp! {{
    #include "iceoryx_posh/testing/roudi_environment/roudi_environment.hpp"

    using iox::roudi::RouDiEnvironment;
}}

cpp_class!(pub unsafe struct RouDiEnvironment as "RouDiEnvironment");

impl RouDiEnvironment {
    pub fn new() -> Box<Self> {
        unsafe {
            let raw = cpp!([] -> *mut RouDiEnvironment as "RouDiEnvironment*"
            {
                return new RouDiEnvironment();
            });

            Box::from_raw(raw)
        }
    }
}
