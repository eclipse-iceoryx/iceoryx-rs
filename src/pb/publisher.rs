// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: © Contributors to the iceoryx-rs project
// SPDX-FileContributor: Mathias Kraus

use super::sample::SampleMut;
use crate::marker::ShmSend;
use crate::ConsumerTooSlowPolicy;
use crate::IceoryxError;

use std::marker::PhantomData;
use std::mem::MaybeUninit;

pub struct PublisherBuilder<'a, T: ShmSend> {
    service: &'a str,
    instance: &'a str,
    event: &'a str,
    options: ffi::PublisherOptions,
    phantom: PhantomData<T>,
}

impl<'a, T: ShmSend> PublisherBuilder<'a, T> {
    pub fn new(service: &'a str, instance: &'a str, event: &'a str) -> Self {
        Self {
            service,
            instance,
            event,
            options: ffi::PublisherOptions::default(),
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

    pub fn subscriber_too_slow_policy(
        mut self,
        subscriber_too_slow_policy: ConsumerTooSlowPolicy,
    ) -> Self {
        self.options.subscriber_too_slow_policy = subscriber_too_slow_policy;
        self
    }

    pub fn create(mut self) -> Result<Publisher<T>, IceoryxError> {
        self.options.offer_on_create = true;
        let ffi_pub = ffi::Publisher::new(self.service, self.instance, self.event, &self.options)
            .ok_or(IceoryxError::PublisherCreationFailed)?;

        Ok(Publisher {
            ffi_pub,
            phantom: PhantomData,
        })
    }

    pub fn create_without_offer(mut self) -> Result<InactivePublisher<T>, IceoryxError> {
        self.options.offer_on_create = false;
        let ffi_pub = ffi::Publisher::new(self.service, self.instance, self.event, &self.options)
            .ok_or(IceoryxError::PublisherCreationFailed)?;

        Ok(InactivePublisher {
            ffi_pub,
            phantom: PhantomData,
        })
    }
}

pub struct InactivePublisher<T: ShmSend> {
    ffi_pub: Box<ffi::Publisher>,
    phantom: PhantomData<T>,
}

impl<T: ShmSend> InactivePublisher<T> {
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

pub struct Publisher<T: ShmSend> {
    ffi_pub: Box<ffi::Publisher>,
    phantom: PhantomData<T>,
}

impl<T: ShmSend> Publisher<T> {
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

    pub fn publish(&self, mut sample: SampleMut<T>) {
        if let Some(chunk) = sample.data.take() {
            sample.service.ffi_pub.send_chunk(chunk)
        }
    }

    pub(super) fn release_chunk(&self, chunk: Box<T>) {
        self.ffi_pub.free_chunk(chunk);
    }
}

impl<T: ShmSend + Default> Publisher<T> {
    pub fn allocate_sample(&self) -> Result<SampleMut<T>, IceoryxError> {
        let mut data = self
            .ffi_pub
            .allocate_chunk::<T>()
            .ok_or(IceoryxError::SampleAllocationFailed)?;

        // TDDO use this once 'new_uninit' is stabilized
        // let data = Box::write(data, T::default());
        // until then, the transmute is not nice but safe since MaybeUninit has the same layout as the inner type
        (*data).write(T::default());
        let data = unsafe { std::mem::transmute::<Box<MaybeUninit<T>>, Box<T>>(data) };

        Ok(SampleMut {
            data: Some(data),
            service: self,
        })
    }
}

impl<T: ShmSend> Publisher<T> {
    pub fn allocate_sample_uninitialized(&self) -> Result<SampleMut<MaybeUninit<T>>, IceoryxError> {
        let data = self
            .ffi_pub
            .allocate_chunk::<T>()
            .ok_or(IceoryxError::SampleAllocationFailed)?;

        Ok(SampleMut {
            data: Some(data),
            service: unsafe {
                // the transmute is not nice but save since MaybeUninit has the same layout as the inner type
                std::mem::transmute::<&Publisher<T>, &Publisher<MaybeUninit<T>>>(self)
            },
        })
    }
}
