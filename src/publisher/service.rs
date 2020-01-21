// Copyright 2020 Mathias Kraus - All rights reserved
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// http://www.apache.org/licenses/LICENSE-2.0>. This file may not be
// copied, modified, or distributed except according to those terms.

use super::{
    sample::{SampleMut, POD},
    Publisher,
};
use crate::IceOryxError;

pub struct Service<T: POD> {
    publisher: Publisher<T>,
}

impl<T: POD> Service<T> {
    pub(super) fn new(publisher: Publisher<T>) -> Self {
        Self { publisher }
    }

    pub fn stop(self) -> Publisher<T> {
        self.publisher.ffi_pub.stop_offer();
        self.publisher
    }

    pub fn has_subscribers(&self) -> bool {
        self.publisher.ffi_pub.has_subscribers()
    }

    pub fn allocate_sample(&self) -> Result<SampleMut<T>, IceOryxError> {
        Ok(SampleMut {
            data: Some(self.publisher.ffi_pub.allocate_chunk()?),
            service: &self,
        })
    }

    pub fn publish(&self, mut sample: SampleMut<T>) {
        if let Some(chunk) = sample.data.take() {
            sample.service.publisher.ffi_pub.send_chunk(chunk)
        }
    }

    pub(super) fn release_chunk(&self, chunk: Box<T>) {
        self.publisher.ffi_pub.free_chunk(chunk);
    }
}
