// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: © Contributors to the iceoryx-rs project
// SPDX-FileContributor: Mathias Kraus

use super::Publisher;
use crate::marker::ShmSend;

use std::mem::MaybeUninit;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

/// An owned, reference-counted mutable sample owned by a single publisher
pub struct SampleMutArc<T: ShmSend + ?Sized> {
    pub(super) data: Option<Box<T>>,
    pub(super) publisher: Arc<Publisher<T>>,
}

impl<T: ShmSend + ?Sized> Deref for SampleMutArc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // this is safe since only `drop` and `Publisher::send` will take the `Option`
        unsafe { self.data.as_ref().unwrap_unchecked() }
    }
}

impl<T: ShmSend + ?Sized> DerefMut for SampleMutArc<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // this is safe since only `drop` and `Publisher::send` will take the `Option`
        unsafe { self.data.as_mut().unwrap_unchecked() }
    }
}

impl<T: ShmSend + ?Sized> Drop for SampleMutArc<T> {
    fn drop(&mut self) {
        if let Some(data) = self.data.take() {
            self.publisher.release_chunk(data);
        }
    }
}

impl<T: ShmSend> SampleMutArc<MaybeUninit<T>> {
    /// Extracts the value of `MaybeUninit<T>` container and labels the sample as initialized
    ///
    /// # Safety
    ///
    /// The caller must ensure that `MaybeUninit<T>` really is initialized. Calling this when
    /// the content is not fully initialized causes immediate undefined behavior.
    pub unsafe fn assume_init(mut self) -> SampleMutArc<T> {
        let data = self.data.take().unwrap();

        // TDDO use this once 'new_uninit' is stabilized
        // 'let data = Box::assume_init(data);' or just 'let data = data.assume_init();'
        // until then, the transmute is not nice but safe since MaybeUninit has the same layout as the inner type
        let data = std::mem::transmute::<Box<MaybeUninit<T>>, Box<T>>(data);

        SampleMutArc {
            data: Some(data),
            // the transmute is not nice but safe since MaybeUninit has the same layout as the inner type
            publisher: std::mem::transmute::<Arc<Publisher<MaybeUninit<T>>>, Arc<Publisher<T>>>(
                self.publisher.clone(),
            ),
        }
    }
}

impl<T: ShmSend> SampleMutArc<[MaybeUninit<T>]> {
    /// Extracts the value of `MaybeUninit<T>` container and labels the sample as initialized
    ///
    /// # Safety
    ///
    /// The caller must ensure that `MaybeUninit<T>` really is initialized. Calling this when
    /// the content is not fully initialized causes immediate undefined behavior.
    pub unsafe fn assume_init(mut self) -> SampleMutArc<[T]> {
        let data = self.data.take().unwrap();

        // TDDO use this once 'new_uninit' is stabilized
        // 'let data = Box::assume_init(data);' or just 'let data = data.assume_init();'
        // until then, the transmute is not nice but safe since MaybeUninit has the same layout as the inner type
        let data = std::mem::transmute::<Box<[MaybeUninit<T>]>, Box<[T]>>(data);

        SampleMutArc {
            data: Some(data),
            // the transmute is not nice but safe since MaybeUninit has the same layout as the inner type
            publisher: std::mem::transmute::<Arc<Publisher<[MaybeUninit<T>]>>, Arc<Publisher<[T]>>>(
                self.publisher.clone(),
            ),
        }
    }
}

impl SampleMutArc<[MaybeUninit<u8>]> {
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
            // this is safe since only `drop` and `Publisher::send` will take the `Option`
            self.data.as_mut().unwrap_unchecked(),
        )
    }

    /// Get a mutable reference to an uninitialized T
    ///
    /// If the size and alignment of T do not match with the alignment of the underlying buffer, None is returned.
    pub fn try_as_uninit<T: ShmSend>(&mut self) -> Option<&mut MaybeUninit<T>> {
        unsafe {
            let data = self.data.as_mut().unwrap_unchecked();
            let chunk_header =
                ffi::ChunkHeader::from_user_payload(&*(data.as_ptr())).unwrap_unchecked();
            let payload_size = chunk_header.get_user_payload_size() as usize;
            let payload_alignment = chunk_header.get_user_payload_alignment() as usize;

            if payload_size >= std::mem::size_of::<T>()
                && payload_alignment >= std::mem::align_of::<T>()
            {
                Some(
                    &mut *(std::mem::transmute::<*mut MaybeUninit<u8>, *mut MaybeUninit<T>>(
                        data.as_mut_ptr(),
                    )),
                )
            } else {
                None
            }
        }
    }
}
