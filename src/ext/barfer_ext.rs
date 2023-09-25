use crate::Barfer;

macro_rules! numbers {
    [$($endianness:literal => $fn:ident <> $back:ident: [$($in:ty => $name:ident),*$(,)?]),*$(,)?] => {$($(
        #[doc = concat!("Add an [`", stringify!($in), "`] into the buffer as ", $endianness, "-endian bytes")]
        ///
        /// # Examples
        /// ```
        /// # use barf::ByteBarferExt;
        /// let mut buf: Vec<u8> = Vec::new();
        #[doc = concat!("buf.", stringify!($name), "(42_", stringify!($in) ,");")]
        ///
        #[doc = concat!("assert_eq!(", stringify!($in), "::", stringify!($back), "(buf.as_slice().try_into().unwrap()), 42_", stringify!($in), ");")]
        /// ```
        #[inline]
        fn $name(&mut self, num: $in) -> Result<(), <Self as Barfer<u8>>::Error> {
            self.many(num.$fn())
        }
    )*)*};
}

/// Extensions for [`Barfer<u8>`].
pub trait ByteBarferExt: Barfer<u8> {
    /// Add an [`AsRef<str>`] into the buffer as bytes.
    ///
    /// [`Barfer::slice`] should work for most of the types this function accepts.
    ///
    /// # Examples
    /// ```
    /// # use barf::ByteBarferExt;
    /// let mut buf: Vec<u8> = Vec::new();
    /// buf.string("test");
    ///
    /// assert_eq!(&buf.as_slice(), b"test");
    /// ```
    #[inline]
    fn string<S: AsRef<str>>(&mut self, s: S) -> Result<(), <Self as Barfer<u8>>::Error> {
        self.slice(s.as_ref().as_bytes())
    }

    /// Add a [`char`] into the buffer as bytes.
    ///
    /// [`Barfer::slice`] should work for most of the types this function accepts.
    ///
    /// # Examples
    /// ```
    /// # use barf::ByteBarferExt;
    /// let mut buf: Vec<u8> = Vec::new();
    /// buf.char('a');
    ///
    /// assert_eq!(buf[0], b'a');
    /// ```
    #[inline]
    fn char(&mut self, c: char) -> Result<(), <Self as Barfer<u8>>::Error> {
        let mut buf = [0_u8; 4];
        c.encode_utf8(&mut buf);
        self.slice(&buf[..c.len_utf8()])
    }

    /// Add an [`u64`] into the buffer as [LEB128](https://en.wikipedia.org/wiki/LEB128) encoded bytes.
    ///
    /// # Examples
    /// ```
    /// # use barf::ByteBarferExt;
    /// let mut buf: Vec<u8> = Vec::new();
    /// buf.uleb128(314159);
    ///
    /// assert_eq!(&buf.as_slice(), [175, 150, 19]);
    /// ```
    #[cfg(feature = "leb128")]
    fn uleb128(&mut self, num: u64) -> Result<(), <Self as Barfer<u8>>::Error> {
        let mut encoded = [0; 10];
        let len = unsafe {
            nano_leb128::ULEB128::from(num)
                .write_into(&mut encoded)
                .unwrap_unchecked()
        };

        self.slice(&encoded[..len])
    }

    /// Add an [`i64`] into the buffer as [LEB128](https://en.wikipedia.org/wiki/LEB128) encoded bytes.
    ///
    /// # Examples
    /// ```
    /// # use barf::ByteBarferExt;
    /// let mut buf: Vec<u8> = Vec::new();
    /// buf.uleb128(314159);
    ///
    /// assert_eq!(&buf.as_slice(), [175, 150, 19]);
    /// ```
    #[cfg(feature = "leb128")]
    fn sleb128(&mut self, num: i64) -> Result<(), <Self as Barfer<u8>>::Error> {
        let mut encoded = [0; 10];
        let len = unsafe {
            nano_leb128::SLEB128::from(num)
                .write_into(&mut encoded)
                .unwrap_unchecked()
        };

        self.slice(&encoded[..len])
    }

    /// Add an [`u64`] into the buffer as [vint64](https://crates.io/crates/vint64) encoded bytes.
    ///
    /// # Examples
    /// ```
    /// # use barf::ByteBarferExt;
    /// let mut buf: Vec<u8> = Vec::new();
    /// buf.svint64(314159);
    ///
    /// assert_eq!(&buf.as_slice(), [244, 178, 76]);
    /// ```
    #[cfg(feature = "vint64")]
    fn uvint64(&mut self, num: u64) -> Result<(), <Self as Barfer<u8>>::Error> {
        let encoded = vint64::encode(num);
        self.slice(encoded)
    }

    /// Add an [`i64`] into the buffer as [vint64](https://crates.io/crates/vint64) encoded bytes.
    ///
    /// # Examples
    /// ```
    /// # use barf::ByteBarferExt;
    /// let mut buf: Vec<u8> = Vec::new();
    /// buf.svint64(314159);
    ///
    /// assert_eq!(&buf.as_slice(), [244, 178, 76]);
    /// ```
    #[cfg(feature = "vint64")]
    fn svint64(&mut self, num: i64) -> Result<(), <Self as Barfer<u8>>::Error> {
        let encoded = vint64::signed::encode(num);
        self.slice(encoded)
    }

    numbers![
        "little" => to_le_bytes <> from_le_bytes: [
            u8 => le_u8,
            u16 => le_u16,
            u32 => le_u32,
            u64 => le_u64,
            u128 => le_u128,
            i8 => le_i8,
            i16 => le_i16,
            i32 => le_i32,
            i64 => le_i64,
            i128 => le_i128,
            f32 => le_f32,
            f64 => le_f64,
        ],
        "big" => to_be_bytes <> from_be_bytes: [
            u8 => be_u8,
            u16 => be_u16,
            u32 => be_u32,
            u64 => be_u64,
            u128 => be_u128,
            i8 => be_i8,
            i16 => be_i16,
            i32 => be_i32,
            i64 => be_i64,
            i128 => be_i128,
            f32 => be_f32,
            f64 => be_f64,
        ],
        "native" => to_ne_bytes <> from_ne_bytes: [
            u8 => ne_u8,
            u16 => ne_u16,
            u32 => ne_u32,
            u64 => ne_u64,
            u128 => ne_u128,
            i8 => ne_i8,
            i16 => ne_i16,
            i32 => ne_i32,
            i64 => ne_i64,
            i128 => ne_i128,
            f32 => ne_f32,
            f64 => ne_f64,
        ],
    ];
}

impl<B: Barfer<u8>> ByteBarferExt for B {}

/// Extensions for [`Barfer<char>`].
pub trait CharBarferExt: Barfer<char> {
    /// Add an [`AsRef<str>`] into the buffer as [`char`]s.
    ///
    /// # Examples
    /// ```
    /// # use barf::CharBarferExt;
    /// let mut buf: Vec<char> = Vec::new();
    /// buf.string("test");
    ///
    /// assert_eq!(&buf.as_slice(), &['t', 'e', 's', 't']);
    /// ```
    #[inline]
    fn string<S: AsRef<str>>(&mut self, s: S) -> Result<(), <Self as Barfer<char>>::Error> {
        self.many(s.as_ref().chars())
    }
}

impl<B: Barfer<char>> CharBarferExt for B {}
