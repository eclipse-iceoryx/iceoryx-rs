// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: © Contributors to the iceoryx-rs project
// SPDX-FileContributor: Mathias Kraus

use super::{Publisher, RawSampleMut};
use crate::marker::ShmSend;

use std::mem::MaybeUninit;
use std::ops::{Deref, DerefMut};

/// A mutable sample owned by a single publisher
pub struct SampleMut<'a, T: ShmSend + ?Sized> {
    data: RawSampleMut<T>,
    publisher: &'a Publisher<T>,
}

impl<'a, T: ShmSend + ?Sized> Deref for SampleMut<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: `as_payload_ptr` returns a non-null ptr
        unsafe { &*self.data.as_payload_ptr() }
    }
}

impl<'a, T: ShmSend + ?Sized> DerefMut for SampleMut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: `as_payload_mut_ptr` returns a non-null ptr
        unsafe { &mut *self.data.as_payload_mut_ptr() }
    }
}

impl<'a, T: ShmSend + ?Sized> Drop for SampleMut<'a, T> {
    fn drop(&mut self) {
        self.publisher.release_raw(self.data);
    }
}

impl<'a, T: ShmSend + ?Sized> SampleMut<'a, T> {
    pub(super) fn new(data: RawSampleMut<T>, publisher: &'a Publisher<T>) -> Self {
        Self { data, publisher }
    }

    fn into_raw_parts(self) -> (RawSampleMut<T>, &'a Publisher<T>) {
        let sample = self.data;
        let publisher = self.publisher;
        std::mem::forget(self); // forget `self` to not call drop
        (sample, publisher)
    }

    /// Convert into `RawSampleMut`
    ///
    /// The sample must manually be managed by calling either `publish_raw` or `release_raw`
    /// via the corresponding publisher. Not doing either will lead to a memory leak.
    /// `RawSampleMut` is a thin wrapper around a raw pointer which is guaranteed to be non-null
    /// but it might be dangling if `publish_raw` or `release_raw` was already called on the
    /// specific raw sample.
    pub fn into_raw(self) -> RawSampleMut<T> {
        let (sample, _) = self.into_raw_parts();
        sample
    }
}

impl<'a, T: ShmSend> SampleMut<'a, MaybeUninit<T>> {
    /// Extracts the value of `MaybeUninit<T>` container and labels the sample as initialized
    ///
    /// # Safety
    ///
    /// The caller must ensure that `MaybeUninit<T>` really is initialized. Calling this when
    /// the content is not fully initialized causes immediate undefined behavior.
    pub unsafe fn assume_init(self) -> SampleMut<'a, T> {
        let (data, publisher) = self.into_raw_parts();

        // the transmute is not nice but safe since MaybeUninit has the same layout as the inner type
        let data = std::mem::transmute::<RawSampleMut<MaybeUninit<T>>, RawSampleMut<T>>(data);
        let publisher = std::mem::transmute::<&Publisher<MaybeUninit<T>>, &Publisher<T>>(publisher);

        SampleMut { data, publisher }
    }
}

impl<'a, T: ShmSend> SampleMut<'a, [MaybeUninit<T>]> {
    /// Extracts the value of `MaybeUninit<T>` container and labels the sample as initialized
    ///
    /// # Safety
    ///
    /// The caller must ensure that `MaybeUninit<T>` really is initialized. Calling this when
    /// the content is not fully initialized causes immediate undefined behavior.
    pub unsafe fn assume_init(self) -> SampleMut<'a, [T]> {
        let (data, publisher) = self.into_raw_parts();

        // the transmute is not nice but safe since MaybeUninit has the same layout as the inner type
        let data = std::mem::transmute::<RawSampleMut<[MaybeUninit<T>]>, RawSampleMut<[T]>>(data);
        let publisher =
            std::mem::transmute::<&Publisher<[MaybeUninit<T>]>, &Publisher<[T]>>(publisher);

        SampleMut { data, publisher }
    }
}

impl<'a> SampleMut<'a, [MaybeUninit<u8>]> {
    /// Get a mutable slice to the elements
    ///
    /// # Safety
    ///
    /// It is safe to write to the slice but reading is undefined behaviour.
    /// The main purpose of this method is to be used in combination with the `BufMut` trait of the
    /// [bytes](https://crates.io/crates/bytes) crate.
    pub unsafe fn slice_assume_init_mut(&mut self) -> &mut [u8] {
        // TODO check if `MaybeUninit::slice_assume_init_mut` can be used once it is stabilized;
        // it might not be possible since current documentation labels the usage as undefined behavior
        // without restricting it to read access
        std::mem::transmute::<&mut [MaybeUninit<u8>], &mut [u8]>(
            // `as_payload_ptr` returns a non-null ptr
            &mut *self.data.as_payload_mut_ptr(),
        )
    }

    /// Get a mutable reference to an uninitialized T
    ///
    /// If the size and alignment of T do not match with the alignment of the underlying buffer, None is returned.
    pub fn try_as_uninit<T: ShmSend>(&mut self) -> Option<&mut MaybeUninit<T>> {
        unsafe {
            let chunk_header = self.data.chunk_header();
            let payload_size = chunk_header.get_user_payload_size();
            let payload_alignment = chunk_header.get_user_payload_alignment();

            if payload_size >= std::mem::size_of::<T>()
                && payload_alignment >= std::mem::align_of::<T>()
            {
                let payload = self.data.cast::<MaybeUninit<u8>>().as_payload_mut_ptr();
                Some(
                    &mut *(std::mem::transmute::<*mut MaybeUninit<u8>, *mut MaybeUninit<T>>(
                        payload,
                    )),
                )
            } else {
                None
            }
        }
    }
}
