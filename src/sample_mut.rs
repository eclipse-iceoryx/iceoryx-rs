// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: © Contributors to the iceoryx-rs project
// SPDX-FileContributor: Mathias Kraus

use super::Publisher;
use crate::marker::ShmSend;

use std::mem::MaybeUninit;
use std::ops::{Deref, DerefMut};

pub struct SampleMut<'a, T: ShmSend> {
    pub(super) data: Option<Box<T>>,
    pub(super) service: &'a Publisher<T>,
}

impl<'a, T: ShmSend> Deref for SampleMut<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.data.as_ref().expect("valid sample")
    }
}

impl<'a, T: ShmSend> DerefMut for SampleMut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data.as_mut().expect("valid sample")
    }
}

impl<'a, T: ShmSend> Drop for SampleMut<'a, T> {
    fn drop(&mut self) {
        if let Some(data) = self.data.take() {
            self.service.release_chunk(data);
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
            service: std::mem::transmute::<&Publisher<MaybeUninit<T>>, &Publisher<T>>(self.service),
        }
    }
}
