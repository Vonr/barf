mod sealed {
    pub trait Sealed {}

    #[cfg(all(not(feature = "std"), feature = "alloc"))]
    use alloc::string::String;
    #[cfg(any(feature = "std", feature = "alloc"))]
    impl Sealed for String {}
}

/// Extensions for barfing into a [`String`].
#[cfg(any(feature = "std", feature = "alloc"))]
pub trait StringBarfExt: sealed::Sealed {
    /// Add an [`AsRef<str>`] into the string.
    ///
    /// # Examples
    /// ```
    /// # use barf::StringBarfExt;
    /// let mut buf = String::new();
    /// buf.string("test");
    ///
    /// assert_eq!(buf, "test");
    /// ```
    fn string<S: AsRef<str>>(&mut self, s: S);

    /// Add an [`AsRef<[u8]>`] into the string after UTF-8 validation.
    ///
    /// # Examples
    /// ```
    /// # use barf::StringBarfExt;
    /// let mut buf = String::new();
    /// buf.bytes(b"test").unwrap();
    ///
    /// assert_eq!(buf, "test");
    /// ```
    fn bytes<B: AsRef<[u8]>>(&mut self, b: B) -> Result<(), core::str::Utf8Error>;
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl StringBarfExt for String {
    #[inline]
    fn string<S: AsRef<str>>(&mut self, s: S) {
        self.push_str(s.as_ref());
    }

    fn bytes<B: AsRef<[u8]>>(&mut self, b: B) -> Result<(), core::str::Utf8Error> {
        self.push_str(core::str::from_utf8(b.as_ref())?);
        Ok(())
    }
}
