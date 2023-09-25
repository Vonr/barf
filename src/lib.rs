#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::string::String;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

use core::clone::Clone;
use core::iter::IntoIterator;
use core::result::Result;

mod stack_buffer;
pub use stack_buffer::*;

mod ext;
pub use ext::*;

/// A [`Barfer`] is a trait that wraps buffers for use with this crate, `barf`.
///
/// It exposes an associated Error type - usually [`core::convert::Infallible`], and functions to
/// interact with the buffer.
///
/// - [`Barfer::single`]: Add a single value of type `T` into the buffer.
/// - [`Barfer::many`]: Add an [`IntoIterator`] of values into the buffer.
/// - [`Barfer::slice`]: Add an [`AsRef<T>`] of values into the buffer, only exists if `T` implements [`Clone`].
pub trait Barfer<T> {
    /// Error type of the three functions, usually [`core::convert::Infallible`].
    ///
    /// Could be used to express errors such as not having enough capacity to fit new values.
    type Error;

    /// Add a single value to the buffer.
    ///
    /// # Examples
    /// ```
    /// # use barf::Barfer;
    /// let mut buf: Vec<u8> = Vec::new();
    /// buf.single(42);
    ///
    /// assert_eq!(buf[0], 42);
    /// ```
    fn single(&mut self, value: T) -> Result<(), Self::Error>;

    /// Add a single value of type `T` into the buffer.
    ///
    /// # Examples
    /// ```
    /// # use barf::Barfer;
    /// let mut buf: Vec<u8> = Vec::new();
    /// buf.many([1, 2, 3]);
    ///
    /// assert_eq!(buf.as_slice(), [1, 2, 3]);
    /// ```
    fn many<I: IntoIterator<Item = T>>(&mut self, iter: I) -> Result<(), Self::Error> {
        for value in iter {
            self.single(value)?;
        }

        Ok(())
    }

    /// Add an [`AsRef<T>`] of values into the buffer.
    ///
    /// Only exists if `T` implements [`Clone`].
    ///
    /// # Examples
    /// ```
    /// # use barf::Barfer;
    /// let mut buf: Vec<u8> = Vec::new();
    /// buf.slice([1, 2, 3]);
    ///
    /// assert_eq!(buf.as_slice(), [1, 2, 3]);
    /// ```
    fn slice<S: AsRef<[T]>>(&mut self, slice: S) -> Result<(), Self::Error>
    where
        T: Clone;
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<T> Barfer<T> for Vec<T> {
    type Error = core::convert::Infallible;

    #[inline]
    fn single(&mut self, value: T) -> Result<(), Self::Error> {
        self.push(value);
        Ok(())
    }

    #[inline]
    fn many<I: IntoIterator<Item = T>>(&mut self, iter: I) -> Result<(), Self::Error> {
        self.extend(iter);
        Ok(())
    }

    #[inline]
    fn slice<S: AsRef<[T]>>(&mut self, slice: S) -> Result<(), Self::Error>
    where
        T: Clone,
    {
        self.extend_from_slice(slice.as_ref());
        Ok(())
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl Barfer<char> for String {
    type Error = core::convert::Infallible;

    #[inline]
    fn single(&mut self, value: char) -> Result<(), Self::Error> {
        self.push(value);
        Ok(())
    }

    #[inline]
    fn many<I: IntoIterator<Item = char>>(&mut self, iter: I) -> Result<(), Self::Error> {
        self.extend(iter);
        Ok(())
    }

    #[inline]
    fn slice<S: AsRef<[char]>>(&mut self, slice: S) -> Result<(), Self::Error> {
        let slice = slice.as_ref();
        let mut buf = [0; 4];

        for c in slice {
            self.push_str(c.encode_utf8(&mut buf));
        }
        Ok(())
    }
}
