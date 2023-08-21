// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: © Contributors to the iceoryx-rs project
// SPDX-FileContributor: Mathias Kraus

use crate::ChunkHeader;

use std::ffi::c_void;
use std::fmt;
use std::slice;

/// A `*const T` non-zero sample pointer to the user payload.
#[repr(transparent)]
pub struct RawSample<T: ?Sized> {
    payload: *const T,
}

impl<T: ?Sized> RawSample<T> {
    /// Creates a new `RawSample`.
    ///
    /// # Safety
    ///
    /// `payload` must be non-null.
    #[inline]
    pub unsafe fn new_unchecked(payload: *const T) -> Self {
        debug_assert!(
            !payload.is_null(),
            "RawSample::new_unchecked requires that the payload pointer is non-null"
        );
        Self { payload }
    }

    /// Creates a new `RawSample`.
    #[allow(clippy::not_unsafe_ptr_arg_deref)] // false positive
    #[inline]
    pub fn new(payload: *const T) -> Option<Self> {
        if !payload.is_null() {
            // SAFETY: `payload` pointer is checked to be non-null
            Some(unsafe { Self::new_unchecked(payload) })
        } else {
            None
        }
    }

    /// Casts to a `RawSample` of another type.
    #[must_use]
    #[inline]
    pub fn cast<U>(self) -> RawSample<U> {
        // SAFETY: `self.as_payload_ptr` returns a non-null ptr
        unsafe { RawSample::new_unchecked(self.as_payload_ptr().cast::<U>()) }
    }

    /// Acquires the underlying payload pointer as `*const` pointer.
    #[must_use]
    #[inline(always)]
    pub fn as_payload_ptr(self) -> *const T {
        self.payload
    }

    /// Returns a reference to the `ChunkHeader`.
    #[must_use]
    #[inline]
    pub fn chunk_header(&self) -> &ChunkHeader {
        // SAFTEY: `self.as_payload_ptr` returns a non-null ptr
        unsafe { ChunkHeader::from_user_payload_unchecked(self.as_payload_ptr().cast::<c_void>()) }
    }
}

impl<T> RawSample<[T]> {
    /// Creates a non-null raw slice from a thin payload pointer and a length.
    ///
    /// The `len` argument is the number of **elements**, not the number of bytes.
    ///
    /// This function is safe, but dereferencing the return value is unsafe.
    /// See the documentation of [`slice::from_raw_parts`] for slice safety requirements.
    #[must_use]
    #[inline]
    pub fn slice_from_raw_parts(sample: RawSample<T>, len: usize) -> RawSample<[T]> {
        // SAFETY: `self.as_payload_ptr` returns a non-null ptr
        unsafe { Self::new_unchecked(slice::from_raw_parts(sample.as_payload_ptr(), len)) }
    }

    /// Returns the length of a non-null raw slice.
    #[must_use]
    #[inline]
    pub fn len(self) -> usize {
        // SAFETY: `self.as_payload_ptr` returns a non-null ptr
        unsafe { (*self.as_payload_ptr()).len() }
    }
}

impl<T: ?Sized> Clone for RawSample<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: ?Sized> Copy for RawSample<T> {}

impl<T: ?Sized> fmt::Debug for RawSample<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Pointer::fmt(&self.as_payload_ptr(), f)
    }
}

impl<T: ?Sized> fmt::Pointer for RawSample<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Pointer::fmt(&self.as_payload_ptr(), f)
    }
}

/// A `*mut T` non-zero sample pointer to the user payload.
#[repr(transparent)]
pub struct RawSampleMut<T: ?Sized> {
    payload: *mut T,
}

impl<T: ?Sized> RawSampleMut<T> {
    /// Creates a new `RawSampleMut`.
    ///
    /// # Safety
    ///
    /// `payload` must be non-null.
    #[inline]
    pub unsafe fn new_unchecked(payload: *mut T) -> Self {
        debug_assert!(
            !payload.is_null(),
            "RawSampleMut::new_unchecked requires that the payload pointer is non-null"
        );
        Self { payload }
    }

    /// Creates a new `RawSampleMut`.
    #[allow(clippy::not_unsafe_ptr_arg_deref)] // false positive
    #[inline]
    pub fn new(payload: *mut T) -> Option<Self> {
        if !payload.is_null() {
            // SAFETY: `payload` pointer is checked to be non-null
            Some(unsafe { Self::new_unchecked(payload) })
        } else {
            None
        }
    }

    /// Casts to a `RawSampleMut` of another type.
    #[must_use]
    #[inline]
    pub fn cast<U>(self) -> RawSampleMut<U> {
        // SAFETY: `self.as_payload_mut_ptr` returns a non-null ptr
        unsafe { RawSampleMut::new_unchecked(self.as_payload_mut_ptr().cast::<U>()) }
    }

    /// Acquires the underlying payload pointer as `*const` pointer.
    #[must_use]
    #[inline(always)]
    pub fn as_payload_ptr(self) -> *const T {
        self.as_payload_mut_ptr()
    }

    /// Acquires the underlying payload pointer as `*mut` pointer.
    #[must_use]
    #[inline(always)]
    pub fn as_payload_mut_ptr(self) -> *mut T {
        self.payload
    }

    /// Returns a reference to the `ChunkHeader`.
    #[must_use]
    #[inline]
    pub fn chunk_header(&self) -> &ChunkHeader {
        // SAFTEY: `self.as_payload_ptr` returns a non-null ptr
        unsafe { ChunkHeader::from_user_payload_unchecked(self.as_payload_ptr().cast::<c_void>()) }
    }
}

impl<T> RawSampleMut<[T]> {
    /// Creates a non-null raw slice from a thin payload pointer and a length.
    ///
    /// The `len` argument is the number of **elements**, not the number of bytes.
    ///
    /// This function is safe, but dereferencing the return value is unsafe.
    /// See the documentation of [`slice::from_raw_parts_mut`] for slice safety requirements.
    #[must_use]
    #[inline]
    pub fn slice_from_raw_parts(sample: RawSampleMut<T>, len: usize) -> RawSampleMut<[T]> {
        // SAFETY: `self.as_payload_mut_ptr` returns a non-null ptr
        unsafe { Self::new_unchecked(slice::from_raw_parts_mut(sample.as_payload_mut_ptr(), len)) }
    }

    /// Returns the length of a non-null raw slice.
    #[must_use]
    #[inline]
    pub fn len(self) -> usize {
        // SAFETY: `self.as_payload_ptr` returns a non-null ptr
        unsafe { (*self.as_payload_ptr()).len() }
    }
}

impl<T: ?Sized> Clone for RawSampleMut<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: ?Sized> Copy for RawSampleMut<T> {}

impl<T: ?Sized> fmt::Debug for RawSampleMut<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Pointer::fmt(&self.as_payload_ptr(), f)
    }
}

impl<T: ?Sized> fmt::Pointer for RawSampleMut<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Pointer::fmt(&self.as_payload_ptr(), f)
    }
}
