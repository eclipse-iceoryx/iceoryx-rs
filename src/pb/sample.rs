// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: © Contributors to the iceoryx-rs project
// SPDX-FileContributor: Mathias Kraus

use super::Publisher;

use std::ops::{Deref, DerefMut};

/// # Safety
///
/// This is a marker trait for types that can be transferred via shared memory.
/// The types must satisfy the following conditions:
/// - no heap is used
/// - the data structure is entirely contained in the shared memory - no pointers
///   to process local memory, no references to process local constructs, no dynamic allocators
/// - the data structure has to be relocatable and therefore must not internally
///   use pointers/references
/// In general, types that could implement the Copy trait fulfill these requirements.
pub unsafe trait POD {}
// TODO more impls
unsafe impl POD for i8 {}
unsafe impl POD for u8 {}
unsafe impl POD for i16 {}
unsafe impl POD for u16 {}
unsafe impl POD for i32 {}
unsafe impl POD for u32 {}
unsafe impl POD for i64 {}
unsafe impl POD for u64 {}
unsafe impl POD for f32 {}
unsafe impl POD for f64 {}
unsafe impl POD for isize {}
unsafe impl POD for usize {}

pub struct SampleMut<'a, T: POD> {
    pub(super) data: Option<Box<T>>,
    pub(super) service: &'a Publisher<T>,
}

impl<'a, T: POD> Deref for SampleMut<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.data.as_ref().expect("valid sample")
    }
}

impl<'a, T: POD> DerefMut for SampleMut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data.as_mut().expect("valid sample")
    }
}

impl<'a, T: POD> Drop for SampleMut<'a, T> {
    fn drop(&mut self) {
        if let Some(chunk) = self.data.take() {
            self.service.release_chunk(chunk);
        }
    }
}
