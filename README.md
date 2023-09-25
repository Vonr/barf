# Barf

Turn any input into "barf"-ed output.

The name is a play on the purpose of this crate as a deparser - the opposite of the fantastic [nom](https://github.com/rust-bakery/nom) crate.

Barf aims to have some integrations with common datatypes such as [LEB128 encoded variable-length integers](https://en.wikipedia.org/wiki/LEB128).  
When these integrations require another crate, they will be locked behind feature flags.

These integrations include
- LEB128 with the "leb128" feature flag using [nano-leb128](https://crates.io/crates/nano-leb128).
- [vint64](https://crates.io/crates/vint64) with the "vint64" feature flag.

For a full list, see [Cargo.toml](https://github.com/Vonr/barf/blob/master/Cargo.toml).

If you want to see more integrations, please open an issue, or preferably a pull request.  
There is also nothing stopping you from implementing [`Barfer`] on your own types and creating extension traits for them.

Barf can operate in no_std environments, but the "alloc" feature flag needs to be enabled for the default implementations of [`Barfer`] for [`Vec`] and [`String`].

```rust
use barf::Barfer;

// Vec<T> implements Barfer<T>.
let mut buf: Vec<u8> = Vec::new();

// Push 42_u8
buf.single(42); 
// Push "test".bytes() iterator with `many`
buf.many("test".bytes()); 
// Push 1, 2, and 3
buf.slice([1, 2, 3]); 

assert_eq!(
    &buf[..], 
    [
        42,                 // 42_u8
        116, 101, 115, 116, // Bytes in "test"
        1, 2, 3,            // 1, 2, and 3
    ]
);
```
