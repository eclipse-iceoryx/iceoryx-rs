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
    /// Get a reference to a ChunkHeader
    ///
    /// # Safety
    ///
    /// The caller must ensure that `payload` is non-null
    pub(super) unsafe fn from_user_payload_unchecked<'a>(payload: *const c_void) -> &'a Self {
        unsafe {
            let chunk_header = cpp!([payload as "void*"] -> *const c_void as "const void*" {
                return iox::mepoo::ChunkHeader::fromUserPayload(payload);
            });
            debug_assert!(
                !chunk_header.is_null(),
                "The ChunkHeader ptr should always be non-null when the payload ptr was non-null!"
            );
            &*(chunk_header as *const Self)
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
