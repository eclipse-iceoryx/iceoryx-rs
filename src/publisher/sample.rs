// Copyright 2020 Mathias Kraus - All rights reserved
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// http://www.apache.org/licenses/LICENSE-2.0>. This file may not be
// copied, modified, or distributed except according to those terms.

use super::service::Service;

use std::ops::{Deref, DerefMut};

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
    pub(super) service: &'a Service<T>,
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
        self.data
            .take()
            .map(|chunk| self.service.release_chunk(chunk));
    }
}
