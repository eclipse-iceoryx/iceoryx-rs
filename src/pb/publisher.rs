// Copyright 2020 Mathias Kraus - All rights reserved
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// http://www.apache.org/licenses/LICENSE-2.0>. This file may not be
// copied, modified, or distributed except according to those terms.

use super::{sample::SampleMut, Topic, POD};
use crate::IceOryxError;

pub struct Publisher<T: POD> {
    publisher: Topic<T>,
}

impl<T: POD> Publisher<T> {
    pub(super) fn new(publisher: Topic<T>) -> Self {
        Publisher { publisher }
    }

    pub fn is_offered(&self) -> bool {
        self.publisher.ffi_pub.is_offered()
    }

    pub fn stop(self) -> Topic<T> {
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
