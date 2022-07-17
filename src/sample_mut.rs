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
    pub unsafe fn assume_init(mut self) -> SampleMut<'a, T> {
        let data = self.data.take().unwrap();

        // TDDO use this once 'new_uninit' is stabilized
        // 'let data = Box::assume_init(data);' or just 'let data = data.assume_init();'
        // until then, the transmute is not nice but safe since MaybeUninit has the same layout as the inner type
        let data = std::mem::transmute::<Box<MaybeUninit<T>>, Box<T>>(data);

        SampleMut {
            data: Some(data),
            // the transmute is not nice but save since MaybeUninit has the same layout as the inner type
            publisher: std::mem::transmute::<&Publisher<MaybeUninit<T>>, &Publisher<T>>(
                self.publisher,
            ),
        }
    }
}

impl<'a, T: ShmSend> SampleMut<'a, [MaybeUninit<T>]> {
    pub unsafe fn assume_init(mut self) -> SampleMut<'a, [T]> {
        let data = self.data.take().unwrap();

        // TDDO use this once 'new_uninit' is stabilized
        // 'let data = Box::assume_init(data);' or just 'let data = data.assume_init();'
        // until then, the transmute is not nice but safe since MaybeUninit has the same layout as the inner type
        let data = std::mem::transmute::<Box<[MaybeUninit<T>]>, Box<[T]>>(data);

        SampleMut {
            data: Some(data),
            // the transmute is not nice but save since MaybeUninit has the same layout as the inner type
            publisher: std::mem::transmute::<&Publisher<[MaybeUninit<T>]>, &Publisher<[T]>>(
                self.publisher,
            ),
        }
    }

    pub unsafe fn slice_assume_init_mut(&mut self) -> &mut [T] {
        // TODO use `MaybeUninit::slice_assume_init_mut` once it is stabilized
        std::mem::transmute::<&mut [MaybeUninit<T>], &mut [T]>(
            // this is safe since only `drop` and `Publisher::send` will take the `Option`
            self.data.as_mut().unwrap_unchecked(),
        )
    }
}
