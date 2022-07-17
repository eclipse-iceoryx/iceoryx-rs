// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: © Contributors to the iceoryx-rs project
// SPDX-FileContributor: Mathias Kraus

use super::SampleMut;
use crate::marker::ShmSend;
use crate::ConsumerTooSlowPolicy;
use crate::IceoryxError;

use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::slice::from_raw_parts_mut;

/// Create a publisher with custom options
///
/// # Example
/// ```
/// # use iceoryx_rs::Runtime;
/// use iceoryx_rs::PublisherBuilder;
/// # use ffi::RouDiEnvironment;
/// #
/// # use anyhow::{anyhow, Result};
/// # fn main() -> Result<()> {
/// # let _roudi = RouDiEnvironment::new();
/// #
/// # Runtime::init("basic_pub_sub");
///
/// let publisher = PublisherBuilder::<u32>::new("all", "glory", "hynotoad")
///         .history_capacity(10)
///         .create()?;
/// # Ok(())
/// # }
/// ```
pub struct PublisherBuilder<'a, T: ShmSend + ?Sized> {
    service: &'a str,
    instance: &'a str,
    event: &'a str,
    options: ffi::PublisherOptions,
    phantom: PhantomData<T>,
}

impl<'a, T: ShmSend + ?Sized> PublisherBuilder<'a, T> {
    /// Creates a new `PublisherBuilder`
    ///
    /// The parameter `service`, `instance` and `event` are used to specify the name of the service.
    /// In the future this three strings will probably be fused to together in a single string separated
    /// by slashes `/`.
    pub fn new(service: &'a str, instance: &'a str, event: &'a str) -> Self {
        Self {
            service,
            instance,
            event,
            options: ffi::PublisherOptions::default(),
            phantom: PhantomData,
        }
    }

    /// The size of the buffer for history requests
    ///
    /// Subscriber with a history request will get their samples from the buffer with the size specified in this method.
    ///
    /// By default the history capacity is 0.
    pub fn history_capacity(mut self, size: u64) -> Self {
        self.options.history_capacity = size;
        self
    }

    /// The name of the node where the subscriber should belong to
    ///
    /// Setting the node name has currently no functionality but this might change in future.
    pub fn node_name(mut self, name: String) -> Self {
        self.options.node_name = name;
        self
    }

    /// Sets the policy on how to proceed when the subscriber is too slow in processing the published samples
    ///
    /// By default the oldest samples are removed from the subscriber queue and the latest ones added.
    pub fn subscriber_too_slow_policy(
        mut self,
        subscriber_too_slow_policy: ConsumerTooSlowPolicy,
    ) -> Self {
        self.options.subscriber_too_slow_policy = subscriber_too_slow_policy;
        self
    }

    /// Create a new [`Publisher`]
    ///
    /// The publisher is in the offer state when this method returns. If there are subscriber waiting
    /// to be subscribed, they will be subscribed and samples will have been delivered according to
    /// the history request.
    ///
    /// # Panics
    ///
    /// [`Runtime::init`](crate::Runtime::init) must have been called otherwise this method will panic.
    pub fn create(mut self) -> Result<Publisher<T>, IceoryxError> {
        self.options.offer_on_create = true;
        let ffi_pub = ffi::Publisher::new(self.service, self.instance, self.event, &self.options)
            .ok_or(IceoryxError::PublisherCreationFailed)?;

        Ok(Publisher {
            ffi_pub,
            phantom: PhantomData,
        })
    }

    /// Create a new [`InactivePublisher`]
    ///
    /// The new publisher does not offer and is inactive.
    ///
    /// # Panics
    ///
    /// [`Runtime::init`](crate::Runtime::init) must have been called otherwise this method will panic.
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

/// An inactive publisher which does not offer and is not visible to any subscriber
///
/// This can be used for cases where the service is suspended and the publisher/subscriber need to be disconnected.
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

    /// Offers the service of the publisher by consuming the `InactivePublisher` and creating a [`Publisher`]
    ///
    /// Contrary to [`PublisherBuilder::create`] the publisher does not offer immediately after this
    /// method returns and it might take up to 50 milliseconds until `RouDi` runs its discovery loop.
    pub fn offer(self) -> Publisher<T> {
        self.ffi_pub.offer();
        Publisher::new_from_inactive_publisher(self)
    }
}

/// A publisher which is offering its service
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

    /// Check whether the service is already offered
    ///
    /// After [`PublisherBuilder::create`] this will immediately be true but after [`InactivePublisher::offer`]
    /// it might take up to 50 milliseconds until `RouDi` runs its discovery loop and the publisher does actually
    /// offer the service for subscriber.
    pub fn is_offered(&self) -> bool {
        self.ffi_pub.is_offered()
    }

    /// Stops offering the service by consuming the `Publisher` and creating an [`InactivePublisher`]
    ///
    /// All connected subscriber will be disconnected. It might take up to 50 milliseconds until `RouDi` runs its
    /// discovery loop and this takes effect.
    pub fn stop_offer(self) -> InactivePublisher<T> {
        self.ffi_pub.stop_offer();
        InactivePublisher::new_from_publisher(self)
    }

    /// Checks whether there are subscriber for the service of the publisher
    pub fn has_subscribers(&self) -> bool {
        self.ffi_pub.has_subscribers()
    }

    /// Publishes a sample
    pub fn publish(&self, mut sample: SampleMut<T>) {
        if let Some(chunk) = sample.data.take() {
            sample.publisher.ffi_pub.send(Box::into_raw(chunk))
        }
    }

    pub(super) fn release_chunk(&self, chunk: Box<T>) {
        self.ffi_pub.release(Box::into_raw(chunk));
    }
}

impl<T: ShmSend + Default> Publisher<T> {
    /// Loan a sample
    ///
    /// The loaned sample is initialized with the default value of the type. If this is not desired
    /// or the type does not implement the `Default` trait, [`loan_uninit`](Self::loan_uninit)
    /// can be used.
    pub fn loan(&self) -> Result<SampleMut<T>, IceoryxError> {
        let mut sample = self.loan_uninit()?;

        unsafe {
            sample.as_mut_ptr().write(T::default());
            Ok(sample.assume_init())
        }
    }
}

impl<T: ShmSend> Publisher<T> {
    /// Loan an uninitialized sample
    ///
    /// Same as [`loan`](Self::loan) but with uninitialized data.
    pub fn loan_uninit(&self) -> Result<SampleMut<MaybeUninit<T>>, IceoryxError> {
        let data = self
            .ffi_pub
            .try_allocate::<T>()
            .ok_or(IceoryxError::LoanSampleFailed)?;

        let data = unsafe { Box::from_raw(data as *mut MaybeUninit<T>) };

        Ok(SampleMut {
            data: Some(data),
            publisher: unsafe {
                // the transmute is not nice but safe since MaybeUninit has the same layout as the inner type
                std::mem::transmute::<&Publisher<T>, &Publisher<MaybeUninit<T>>>(self)
            },
        })
    }
}

impl<T: ShmSend + Default> Publisher<[T]> {
    /// Loan a slice with the same alignment as `T`
    ///
    /// The loaned slice is initialized with the default value of the type. If this is not desired
    /// or the type does not implement the `Default` trait, [`loan_uninit_slice`](Self::loan_uninit_slice)
    /// can be used.
    ///
    /// This method can be used to emulate an untyped sample when a `[u8]` slice is used.
    /// In this case, it might be desirable to use a larger alignment, i.e. the largest
    ///  alignment of the type in the buffer. This is required to utilize crates like
    /// [zerocopy](https://crates.io/crates/zerocopy) for safe zero-copy parsing and serialization.
    /// Please use [`loan_slice_with_alignment`](Self::loan_slice_with_alignment) for this purpose.
    pub fn loan_slice(&self, len: usize) -> Result<SampleMut<[T]>, IceoryxError> {
        self.loan_slice_with_alignment(len, std::mem::align_of::<T>())
    }

    /// Loan a slice with a custom alignment
    ///
    /// The alignment must be greater or equal than the alignment of `T`.
    ///
    /// This method is ideal in combination with crates like [zerocopy](https://crates.io/crates/zerocopy) for
    /// safe zero-copy parsing and serialization.
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
    /// Loan an uninitialized slice with the same alignment as `T`
    ///
    /// Same as [`loan_slice`](Self::loan_slice) but with uninitialized data.
    ///
    /// Have a look at [`loan_slice`](Self::loan_slice) for considerations regarding a custom alignment.
    pub fn loan_uninit_slice(
        &self,
        len: usize,
    ) -> Result<SampleMut<[MaybeUninit<T>]>, IceoryxError> {
        self.loan_uninit_slice_with_alignment(len, std::mem::align_of::<T>())
    }

    /// Loan an uninitialized slice with a custom alignment
    ///
    /// Same as [`loan_slice_with_alignment`](Self::loan_slice_with_alignment) but with uninitialized data.
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

        let data = unsafe {
            let data = from_raw_parts_mut(data as *mut MaybeUninit<T>, len as usize);
            Box::from_raw(data)
        };

        Ok(SampleMut {
            data: Some(data),
            publisher: unsafe {
                // the transmute is not nice but safe since MaybeUninit has the same layout as the inner type
                std::mem::transmute::<&Publisher<[T]>, &Publisher<[MaybeUninit<T>]>>(self)
            },
        })
    }
}
