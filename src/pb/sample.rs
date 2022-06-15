// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: © Contributors to the iceoryx-rs project
// SPDX-FileContributor: Mathias Kraus

use super::Publisher;

use std::ops::{Deref, DerefMut};

pub trait POD {}
// TODO more impls
impl POD for i8 {}
impl POD for u8 {}
impl POD for i16 {}
impl POD for u16 {}
impl POD for i32 {}
impl POD for u32 {}
impl POD for i64 {}
impl POD for u64 {}
impl POD for f32 {}
impl POD for f64 {}
impl POD for isize {}
impl POD for usize {}

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
