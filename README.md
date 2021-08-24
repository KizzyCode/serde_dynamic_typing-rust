[![docs.rs](https://docs.rs/serde_dynamic_typing/badge.svg)](https://docs.rs/serde_dynamic_typing)
[![License BSD-2-Clause](https://img.shields.io/badge/License-BSD--2--Clause-blue.svg)](https://opensource.org/licenses/BSD-2-Clause)
[![License MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![crates.io](https://img.shields.io/crates/v/serde_dynamic_typing.svg)](https://crates.io/crates/serde_dynamic_typing)
[![Download numbers](https://img.shields.io/crates/d/serde_dynamic_typing.svg)](https://crates.io/crates/serde_dynamic_typing)
[![AppVeyor CI](https://ci.appveyor.com/api/projects/status/github/KizzyCode/serde_dynamic_typing-rust?svg=true)](https://ci.appveyor.com/project/KizzyCode/serde-dynamic-typing-rust)
[![dependency status](https://deps.rs/crate/serde_dynamic_typing/0.1.0/status.svg)](https://deps.rs/crate/serde_dynamic_typing/0.1.0)

# `serde_dynamic_typing`
Welcome to `serde_dynamic_typing` 🎉

This crate implements a simplified `serde`-compatible type model which offers a simplified representation of various
Rust data structures and allows dynamic runtime inspection. The type model can also be used as intermediate
representation for custom serializers etc.

## Why?
Since the Rust compiler erases most type information, it makes runtime inspection and modification basically impossible.
There are some powerful ways such as the `Any`-trait or `serde`, however the first is mostly useful to implement dynamic
interfaces whereas the latter is designed to offer (de-)serialization of predefined static data structures.

`serde_dynamic_typing` sits somewhere in the middle: On one side it behaves similar to `Any` since it offers a
simplified type model which can be used to build and inspect nearly arbitrarily complex data structures dynamically at
runtime. On the other side it strives to be compatible with `serde` and `serde_derive`, so that you can transform
existing Rust types and structs to `AnyValue` and vice-versa.

Furthermore it can also serve as a simple abstraction layer for `serde`: While `serde` is extremely versatile and
efficient, it also requires a more complex type model and implementation. If you just want to write a simple
(de-)serializer without the need for the full efficiency and features `serde` offers, `serde_dynamic_typing` can greatly
simplify the implementation.


## Details and Example
The entire data model model is based around a few core types and an enum-based "generic" value type.

The simplified types are:
 - `Boolean` represents booleans
 - `Bytes` represents byte containers like `Vec<u8>`, `[u8]` or `[u8; N]`
 - `Enumeration` represents a Rust-native `enum` with optional associated values
 - `Float` represents floating point numbers
 - `Integer` represents integer types
 - `Map` represents map like types like `BTreeMap` and is also used to represent `struct`s and `enum`-values with named 
   fields
 - `Sequence` represents sequence types `Vec<...>`, `[...]` or `[...; N]` with arbitrary values and is also used to
   represent `struct`s and `enum`s with tuple fields
 - `Utf8String` represents UTF-8 string types like `String`, `str` or `char`

Furthermore there is a "generic" enum-type `AnyValue` which can hold any value of the types above.

For example, a struct like
```rust
#[derive(Serialize, Deserialize)]
struct ByteContainer {
    #[serde(with = "serde_bytes")]
    bytes: Vec<u8>
}
```
is represented as
```rust
AnyValue::Map([
    (
        AnyValue::Utf8String(Utf8String::from("bytes")),      // <- map key
        AnyValue::Bytes(Bytes::from(/* value of `bytes` */))  // <- associated value
    )
])
```
