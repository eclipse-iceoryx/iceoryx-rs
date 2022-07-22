// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: © Contributors to the iceoryx-rs project
// SPDX-FileContributor: Mathias Kraus

use super::Publisher;
use crate::marker::ShmSend;

use std::mem::MaybeUninit;
use std::ops::{Deref, DerefMut};

pub struct SampleMut<'a, T: ShmSend + ?Sized> {
    pub(super) data: Option<Box<T>>,
    pub(super) publisher: &'a Publisher<T>,
}

impl<'a, T: ShmSend + ?Sized> Deref for SampleMut<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // this is safe since only `drop` and `Publisher::send` will take the `Option`
        unsafe { self.data.as_ref().unwrap_unchecked() }
    }
}

impl<'a, T: ShmSend + ?Sized> DerefMut for SampleMut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // this is safe since only `drop` and `Publisher::send` will take the `Option`
        unsafe { self.data.as_mut().unwrap_unchecked() }
    }
}

impl<'a, T: ShmSend + ?Sized> Drop for SampleMut<'a, T> {
    fn drop(&mut self) {
        if let Some(data) = self.data.take() {
            self.publisher.release_chunk(data);
        }
    }
}

impl<'a, T: ShmSend> SampleMut<'a, MaybeUninit<T>> {
    /// # Safety
    ///
    /// The caller must ensure that `MaybeUninit<T>` really is initialized. Calling this when
    /// the content is not fully initialized causes immediate undefined behavior.
    pub unsafe fn assume_init(mut self) -> SampleMut<'a, T> {
        let data = self.data.take().unwrap();

        // TDDO use this once 'new_uninit' is stabilized
        // 'let data = Box::assume_init(data);' or just 'let data = data.assume_init();'
        // until then, the transmute is not nice but safe since MaybeUninit has the same layout as the inner type
        let data = std::mem::transmute::<Box<MaybeUninit<T>>, Box<T>>(data);

        SampleMut {
            data: Some(data),
            // the transmute is not nice but safe since MaybeUninit has the same layout as the inner type
            publisher: std::mem::transmute::<&Publisher<MaybeUninit<T>>, &Publisher<T>>(
                self.publisher,
            ),
        }
    }
}

impl<'a, T: ShmSend> SampleMut<'a, [MaybeUninit<T>]> {
    /// # Safety
    ///
    /// The caller must ensure that `MaybeUninit<T>` really is initialized. Calling this when
    /// the content is not fully initialized causes immediate undefined behavior.
    pub unsafe fn assume_init(mut self) -> SampleMut<'a, [T]> {
        let data = self.data.take().unwrap();

        // TDDO use this once 'new_uninit' is stabilized
        // 'let data = Box::assume_init(data);' or just 'let data = data.assume_init();'
        // until then, the transmute is not nice but safe since MaybeUninit has the same layout as the inner type
        let data = std::mem::transmute::<Box<[MaybeUninit<T>]>, Box<[T]>>(data);

        SampleMut {
            data: Some(data),
            // the transmute is not nice but safe since MaybeUninit has the same layout as the inner type
            publisher: std::mem::transmute::<&Publisher<[MaybeUninit<T>]>, &Publisher<[T]>>(
                self.publisher,
            ),
        }
    }
}

impl<'a> SampleMut<'a, [MaybeUninit<u8>]> {
    /// # Safety
    ///
    /// It is safe to write to the slice but reading is undefined behaviour.
    /// The main purpose of this method is to be used in combination with the `bytes::BufMut` trait.
    pub unsafe fn slice_assume_init_mut(&mut self) -> &mut [u8] {
        // TODO check if `MaybeUninit::slice_assume_init_mut` can be used once it is stabilized;
        // it might not be possible since current documentation labels the usage as undefined behavior
        // without restricting it to read access
        std::mem::transmute::<&mut [MaybeUninit<u8>], &mut [u8]>(
            // this is safe since only `drop` and `Publisher::send` will take the `Option`
            self.data.as_mut().unwrap_unchecked(),
        )
    }

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
