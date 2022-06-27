// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: © Contributors to the iceoryx-rs project
// SPDX-FileContributor: Mathias Kraus

use super::Publisher;
use crate::marker::ShmSend;

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
        if let Some(chunk) = self.data.take() {
            self.service.release_chunk(chunk);
        }
    }
}
