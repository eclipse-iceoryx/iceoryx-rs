// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: © Contributors to the iceoryx-rs project
// SPDX-FileContributor: Mathias Kraus

use super::{ffi::Publisher as FfiPublisher, sample::SampleMut, PublisherOptions, POD};
use crate::IceoryxError;

use std::marker::PhantomData;

pub struct PublisherBuilder<'a, T: POD> {
    service: &'a str,
    instance: &'a str,
    event: &'a str,
    options: PublisherOptions,
    phantom: PhantomData<T>,
}

impl<'a, T: POD> PublisherBuilder<'a, T> {
    pub fn new(service: &'a str, instance: &'a str, event: &'a str) -> Self {
        Self {
            service,
            instance,
            event,
            options: PublisherOptions::default(),
            phantom: PhantomData,
        }
    }

    pub fn history_capacity(mut self, history_capacity: u64) -> Self {
        self.options.history_capacity = history_capacity;
        self
    }

    pub fn node_name(mut self, node_name: String) -> Self {
        self.options.node_name = node_name;
        self
    }

    pub fn create(mut self) -> Result<Publisher<T>, IceoryxError> {
        self.options.offer_on_create = true;
        let ffi_pub = FfiPublisher::new(self.service, self.instance, self.event, &self.options)
            .ok_or(IceoryxError::PublisherCreationFailed)?;

        Ok(Publisher {
            ffi_pub,
            phantom: PhantomData,
        })
    }

    pub fn create_without_offer(mut self) -> Result<InactivePublisher<T>, IceoryxError> {
        self.options.offer_on_create = false;
        let ffi_pub = FfiPublisher::new(self.service, self.instance, self.event, &self.options)
            .ok_or(IceoryxError::PublisherCreationFailed)?;

        Ok(InactivePublisher {
            ffi_pub,
            phantom: PhantomData,
        })
    }
}

pub struct InactivePublisher<T: POD> {
    ffi_pub: Box<FfiPublisher>,
    phantom: PhantomData<T>,
}

impl<T: POD> InactivePublisher<T> {
    fn new_from_publisher(publisher: Publisher<T>) -> Self {
        Self {
            ffi_pub: publisher.ffi_pub,
            phantom: PhantomData,
        }
    }

    pub fn offer(self) -> Publisher<T> {
        self.ffi_pub.offer();
        Publisher::new_from_inactive_publisher(self)
    }
}

pub struct Publisher<T: POD> {
    ffi_pub: Box<FfiPublisher>,
    phantom: PhantomData<T>,
}

impl<T: POD> Publisher<T> {
    fn new_from_inactive_publisher(publisher: InactivePublisher<T>) -> Self {
        Self {
            ffi_pub: publisher.ffi_pub,
            phantom: PhantomData,
        }
    }

    pub fn is_offered(&self) -> bool {
        self.ffi_pub.is_offered()
    }

    pub fn stop(self) -> InactivePublisher<T> {
        self.ffi_pub.stop_offer();
        InactivePublisher::new_from_publisher(self)
    }

    pub fn has_subscribers(&self) -> bool {
        self.ffi_pub.has_subscribers()
    }

    pub fn allocate_sample(&self) -> Result<SampleMut<T>, IceoryxError> {
        Ok(SampleMut {
            data: Some(self.ffi_pub.allocate_chunk()?),
            service: self,
        })
    }

    pub fn publish(&self, mut sample: SampleMut<T>) {
        if let Some(chunk) = sample.data.take() {
            sample.service.ffi_pub.send_chunk(chunk)
        }
    }

    pub(super) fn release_chunk(&self, chunk: Box<T>) {
        self.ffi_pub.free_chunk(chunk);
    }
}
