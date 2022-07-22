// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: © Contributors to the iceoryx-rs project
// SPDX-FileContributor: Mathias Kraus

use std::ffi::c_void;

cpp! {{
    #include "iceoryx_posh/mepoo/chunk_header.hpp"

    using iox::mepoo::ChunkHeader;
}}

cpp_class!(pub unsafe struct ChunkHeader as "ChunkHeader");

impl ChunkHeader {
    pub fn from_user_payload<T>(payload: &T) -> Option<&Self> {
        let payload = payload as *const T as *const c_void;
        unsafe {
            let chunk_header = cpp!([payload as "void*"] -> *const c_void as "const void*" {
                return iox::mepoo::ChunkHeader::fromUserPayload(payload);
            });
            if chunk_header.is_null() {
                None
            } else {
                Some(&*(chunk_header as *const Self))
            }
        }
    }

    pub fn get_user_payload_size(&self) -> u32 {
        unsafe {
            cpp!([self as "ChunkHeader*"] -> u32 as "uint32_t" {
                return self->userPayloadSize();
            })
        }
    }

    pub fn get_user_payload_alignment(&self) -> u32 {
        unsafe {
            cpp!([self as "ChunkHeader*"] -> u32 as "uint32_t" {
                return self->userPayloadAlignment();
            })
        }
    }
}
