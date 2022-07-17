// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: © Contributors to the iceoryx-rs project
// SPDX-FileContributor: Mathias Kraus

use super::SampleMut;
use crate::marker::ShmSend;
use crate::ConsumerTooSlowPolicy;
use crate::IceoryxError;

use std::marker::PhantomData;
use std::mem::MaybeUninit;

pub struct PublisherBuilder<'a, T: ShmSend + ?Sized> {
    service: &'a str,
    instance: &'a str,
    event: &'a str,
    options: ffi::PublisherOptions,
    phantom: PhantomData<T>,
}

impl<'a, T: ShmSend + ?Sized> PublisherBuilder<'a, T> {
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

pub struct InactivePublisher<T: ShmSend + ?Sized> {
    ffi_pub: Box<ffi::Publisher>,
    phantom: PhantomData<T>,
}

impl<T: ShmSend + ?Sized> InactivePublisher<T> {
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

pub struct Publisher<T: ShmSend + ?Sized> {
    ffi_pub: Box<ffi::Publisher>,
    phantom: PhantomData<T>,
}

impl<T: ShmSend + ?Sized> Publisher<T> {
    fn new_from_inactive_publisher(publisher: InactivePublisher<T>) -> Self {
        Self {
            ffi_pub: publisher.ffi_pub,
            phantom: PhantomData,
        }
    }

    pub fn is_offered(&self) -> bool {
        self.ffi_pub.is_offered()
    }

    pub fn stop_offer(self) -> InactivePublisher<T> {
        self.ffi_pub.stop_offer();
        InactivePublisher::new_from_publisher(self)
    }

    pub fn has_subscribers(&self) -> bool {
        self.ffi_pub.has_subscribers()
    }

    pub fn publish(&self, mut sample: SampleMut<T>) {
        if let Some(chunk) = sample.data.take() {
            sample.publisher.ffi_pub.send(chunk)
        }
    }

    pub(super) fn release_chunk(&self, chunk: Box<T>) {
        self.ffi_pub.release(chunk);
    }
}

impl<T: ShmSend + Default> Publisher<T> {
    pub fn loan(&self) -> Result<SampleMut<T>, IceoryxError> {
        let mut sample = self.loan_uninit()?;

        unsafe {
            sample.as_mut_ptr().write(T::default());
            Ok(sample.assume_init())
        }
    }
}

impl<T: ShmSend> Publisher<T> {
    pub fn loan_uninit(&self) -> Result<SampleMut<MaybeUninit<T>>, IceoryxError> {
        let data = self
            .ffi_pub
            .try_allocate::<T>()
            .ok_or(IceoryxError::LoanSampleFailed)?;

        Ok(SampleMut {
            data: Some(data),
            publisher: unsafe {
                // the transmute is not nice but save since MaybeUninit has the same layout as the inner type
                std::mem::transmute::<&Publisher<T>, &Publisher<MaybeUninit<T>>>(self)
            },
        })
    }
}

impl<T: ShmSend + Default> Publisher<[T]> {
    pub fn loan_slice(&self, len: usize) -> Result<SampleMut<[T]>, IceoryxError> {
        self.loan_slice_with_alignment(len, std::mem::align_of::<T>())
    }

    pub fn loan_slice_with_alignment(
        &self,
        len: usize,
        align: usize,
    ) -> Result<SampleMut<[T]>, IceoryxError> {
        let mut sample = self.loan_uninit_slice_with_alignment(len, align)?;

        unsafe {
            // TODO use `MaybeUninit::slice_assume_init_mut` once it is stabilized
            std::mem::transmute::<&mut [MaybeUninit<T>], &mut [T]>(
                sample.data.as_mut().expect("valid sample"),
            )
            .fill_with(|| T::default());

            Ok(sample.assume_init())
        }
    }
}

impl<T: ShmSend> Publisher<[T]> {
    pub fn loan_uninit_slice(
        &self,
        len: usize,
    ) -> Result<SampleMut<[MaybeUninit<T>]>, IceoryxError> {
        self.loan_uninit_slice_with_alignment(len, std::mem::align_of::<T>())
    }

    pub fn loan_uninit_slice_with_alignment(
        &self,
        len: usize,
        align: usize,
    ) -> Result<SampleMut<[MaybeUninit<T>]>, IceoryxError> {
        if align < std::mem::align_of::<T>() {
            return Err(IceoryxError::InvalidAlignment {
                requested: align,
                min_required: std::mem::align_of::<T>(),
            });
        }

        let data = self
            .ffi_pub
            .try_allocate_slice(len as u32, align as u32)
            .ok_or(IceoryxError::LoanSampleFailed)?;

        Ok(SampleMut {
            data: Some(data),
            publisher: unsafe {
                // the transmute is not nice but save since MaybeUninit has the same layout as the inner type
                std::mem::transmute::<&Publisher<[T]>, &Publisher<[MaybeUninit<T>]>>(self)
            },
        })
    }
}
