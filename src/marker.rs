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

unsafe impl ShmSend for bool {}

unsafe impl ShmSend for i8 {}
unsafe impl ShmSend for u8 {}
unsafe impl ShmSend for i16 {}
unsafe impl ShmSend for u16 {}
unsafe impl ShmSend for i32 {}
unsafe impl ShmSend for u32 {}
unsafe impl ShmSend for i64 {}
unsafe impl ShmSend for u64 {}
unsafe impl ShmSend for i128 {}
unsafe impl ShmSend for u128 {}
unsafe impl ShmSend for f32 {}
unsafe impl ShmSend for f64 {}
unsafe impl ShmSend for isize {}
unsafe impl ShmSend for usize {}

unsafe impl ShmSend for char {}

unsafe impl<T: ShmSend, const N: usize> ShmSend for [T; N] {}

unsafe impl<T: ShmSend> ShmSend for Option<T> {}

unsafe impl<T: ShmSend, E: ShmSend> ShmSend for Result<T, E> {}

// TODO create macro to impl ShmSend for tuples
unsafe impl<T1, T2> ShmSend for (T1, T2)
where
    T1: ShmSend,
    T2: ShmSend,
{
}
unsafe impl<T1, T2, T3> ShmSend for (T1, T2, T3)
where
    T1: ShmSend,
    T2: ShmSend,
    T3: ShmSend,
{
}
unsafe impl<T1, T2, T3, T4> ShmSend for (T1, T2, T3, T4)
where
    T1: ShmSend,
    T2: ShmSend,
    T3: ShmSend,
    T4: ShmSend,
{
}
unsafe impl<T1, T2, T3, T4, T5> ShmSend for (T1, T2, T3, T4, T5)
where
    T1: ShmSend,
    T2: ShmSend,
    T3: ShmSend,
    T4: ShmSend,
    T5: ShmSend,
{
}
unsafe impl<T1, T2, T3, T4, T5, T6> ShmSend for (T1, T2, T3, T4, T5, T6)
where
    T1: ShmSend,
    T2: ShmSend,
    T3: ShmSend,
    T4: ShmSend,
    T5: ShmSend,
    T6: ShmSend,
{
}
unsafe impl<T1, T2, T3, T4, T5, T6, T7> ShmSend for (T1, T2, T3, T4, T5, T6, T7)
where
    T1: ShmSend,
    T2: ShmSend,
    T3: ShmSend,
    T4: ShmSend,
    T5: ShmSend,
    T6: ShmSend,
    T7: ShmSend,
{
}
unsafe impl<T1, T2, T3, T4, T5, T6, T7, T8> ShmSend for (T1, T2, T3, T4, T5, T6, T7, T8)
where
    T1: ShmSend,
    T2: ShmSend,
    T3: ShmSend,
    T4: ShmSend,
    T5: ShmSend,
    T6: ShmSend,
    T7: ShmSend,
    T8: ShmSend,
{
}
