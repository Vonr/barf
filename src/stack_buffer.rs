use crate::Barfer;
use core::mem::MaybeUninit;

/// A buffer of `T` allocated on the stack that implements [`Barfer<T>`].
///
/// # Examples
/// ```
/// # use barf::{Barfer, ByteBarferExt, StackBuffer, StackBufferError};
/// let mut buf = StackBuffer::<u8, 10>::new();
/// buf.le_u64(42).expect("Should have enough capacity for 8 bytes.");
///
/// assert_eq!(buf.len(), 8);
/// assert_eq!(buf.get(), &[42, 0, 0, 0, 0, 0, 0, 0]);
/// assert_eq!(u64::from_le_bytes(buf.get().try_into().unwrap()), 42);
///
/// buf.le_u64(42).expect_err("Not enough capacity (8 + 8 > 10)");
///
/// assert_eq!(buf.len(), 8);
/// assert_eq!(buf.get(), &[42, 0, 0, 0, 0, 0, 0, 0]);
/// assert_eq!(u64::from_le_bytes(buf.get().try_into().unwrap()), 42);
///
/// buf.le_u16(42).expect("Should have enough capacity for 2 bytes.");
/// assert_eq!(u16::from_le_bytes(buf.get()[8..].try_into().unwrap()), 42);
/// ```
pub struct StackBuffer<T: Sized, const N: usize> {
    pub(crate) buffer: [MaybeUninit<T>; N],
    pub(crate) len: usize,
}

impl<T, const N: usize> Clone for StackBuffer<T, N>
where
    MaybeUninit<T>: Clone,
{
    fn clone(&self) -> Self {
        Self {
            buffer: self.buffer.clone(),
            len: self.len,
        }
    }
}

impl<T: Copy, const N: usize> Copy for StackBuffer<T, N> {}

/// Error type for [`StackBuffer`]'s implementation of [`Barfer`].
#[derive(Debug, PartialEq)]
pub enum StackBufferError {
    NotEnoughCapacity,
}

impl<T, const N: usize> StackBuffer<T, N> {
    /// Creates a new [`StackBuffer`].
    ///
    /// # Examples
    /// ```
    /// # use barf::StackBuffer;
    /// let buf = StackBuffer::<u8, 32>::new();
    /// assert!(buf.is_empty());
    /// ```
    #[inline]
    pub fn new() -> Self {
        StackBuffer {
            buffer: unsafe { MaybeUninit::uninit().assume_init() },
            len: 0,
        }
    }

    /// Returns the current length of the [`StackBuffer`].
    ///
    /// # Examples
    /// ```
    /// # use barf::StackBuffer;
    /// let mut buf = StackBuffer::<u8, 32>::new();
    /// assert_eq!(buf.len(), 0);
    ///
    /// buf.extend([1, 2, 3]);
    /// assert_eq!(buf.len(), 3);
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns whether the [`StackBuffer`] is currently empty.
    ///
    /// # Examples
    /// ```
    /// # use barf::StackBuffer;
    /// let mut buf = StackBuffer::<u8, 32>::new();
    /// assert!(buf.is_empty());
    ///
    /// buf.push(42);
    /// assert!(!buf.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Adds a value to the [`StackBuffer`].
    ///
    /// # Examples
    /// ```
    /// # use barf::StackBuffer;
    /// let mut buf = StackBuffer::<u8, 1>::new();
    /// assert_eq!(buf.len(), 0);
    ///
    /// buf.push(42);
    /// assert_eq!(buf.len(), 1);
    ///
    /// buf.push(42).expect_err("Not enough capacity");
    /// assert_eq!(buf.len(), 1);
    /// ```
    pub fn push(&mut self, value: T) -> Result<(), StackBufferError> {
        if self.len >= N {
            return Err(StackBufferError::NotEnoughCapacity);
        }

        self.buffer[self.len].write(value);
        self.len += 1;
        Ok(())
    }

    /// Adds values from an [`IntoIterator`] to the [`StackBuffer`].
    ///
    /// # Examples
    /// ```
    /// # use barf::StackBuffer;
    /// let mut buf = StackBuffer::<u8, 4>::new();
    /// assert_eq!(buf.len(), 0);
    ///
    /// buf.extend([1, 2, 3]);
    /// assert_eq!(buf.len(), 3);
    ///
    /// buf.extend([1, 2, 3]).expect_err("Not enough capacity");
    /// assert_eq!(buf.len(), 3);
    /// ```
    pub fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) -> Result<(), StackBufferError> {
        let initial_len = self.len;
        let values = iter.into_iter();
        for value in values.into_iter() {
            if self.len >= N {
                self.len = initial_len;
                return Err(StackBufferError::NotEnoughCapacity);
            }
            self.buffer[self.len].write(value);
            self.len += 1;
        }
        Ok(())
    }

    /// Adds values from an [`AsRef<[T]>`] to the [`StackBuffer`].
    ///
    /// # Examples
    /// ```
    /// # use barf::StackBuffer;
    /// let mut buf = StackBuffer::<u8, 4>::new();
    /// assert_eq!(buf.len(), 0);
    ///
    /// buf.extend_from_slice([1, 2, 3]);
    /// assert_eq!(buf.len(), 3);
    ///
    /// buf.extend_from_slice([1, 2, 3]).expect_err("Not enough capacity");
    /// assert_eq!(buf.len(), 3);
    /// ```
    pub fn extend_from_slice<S: AsRef<[T]>>(&mut self, slice: S) -> Result<(), StackBufferError>
    where
        T: Clone,
    {
        let values = slice.as_ref();
        if self.len + values.len() > N {
            return Err(StackBufferError::NotEnoughCapacity);
        }

        for value in values.iter() {
            self.buffer[self.len].write(value.clone());
            self.len += 1;
        }
        Ok(())
    }

    /// Returns the initialized portion of the [`StackBuffer`].
    ///
    /// # Examples
    /// ```
    /// # use barf::StackBuffer;
    /// let mut buf = StackBuffer::<u8, 32>::new();
    ///
    /// buf.extend_from_slice([1, 2, 3]);
    /// assert_eq!(buf.get(), &[1, 2, 3]);
    /// ```
    #[inline]
    pub fn get(&self) -> &[T] {
        unsafe { &*(&self.buffer[..self.len] as *const [MaybeUninit<T>] as *const [T]) }
    }
}

impl<T, const N: usize> Default for StackBuffer<T, N> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const N: usize> Barfer<T> for StackBuffer<T, N> {
    type Error = StackBufferError;

    #[inline]
    fn single(&mut self, value: T) -> Result<(), Self::Error> {
        self.push(value)
    }

    #[inline]
    fn many<I: IntoIterator<Item = T>>(&mut self, iter: I) -> Result<(), Self::Error> {
        self.extend(iter)
    }

    #[inline]
    fn slice<S: AsRef<[T]>>(&mut self, slice: S) -> Result<(), Self::Error>
    where
        T: Clone,
    {
        self.extend_from_slice(slice)
    }
}
