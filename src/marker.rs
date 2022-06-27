// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: © Contributors to the iceoryx-rs project
// SPDX-FileContributor: Mathias Kraus

/// # Safety
///
/// This is a marker trait for types that can be transferred via shared memory.
/// The types must satisfy the following conditions:
/// - no heap is used
/// - the data structure is entirely contained in the shared memory - no pointers
///   to process local memory, no references to process local constructs, no dynamic allocators
/// - the data structure has to be relocatable and therefore must not internally
///   use pointers/references
/// - the type must not impl Drop; drop will not be called when the memory is released since the
///   memory might be located in a shm segment without write access
/// In general, types that could implement the Copy trait fulfill these requirements.
pub unsafe trait ShmSend {}

// TODO more impls
unsafe impl ShmSend for i8 {}
unsafe impl ShmSend for u8 {}
unsafe impl ShmSend for i16 {}
unsafe impl ShmSend for u16 {}
unsafe impl ShmSend for i32 {}
unsafe impl ShmSend for u32 {}
unsafe impl ShmSend for i64 {}
unsafe impl ShmSend for u64 {}
unsafe impl ShmSend for f32 {}
unsafe impl ShmSend for f64 {}
unsafe impl ShmSend for isize {}
unsafe impl ShmSend for usize {}
