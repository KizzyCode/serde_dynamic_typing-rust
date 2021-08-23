[![docs.rs](https://docs.rs/serde_dynamic_types/badge.svg)](https://docs.rs/serde_dynamic_types)
[![License BSD-2-Clause](https://img.shields.io/badge/License-BSD--2--Clause-blue.svg)](https://opensource.org/licenses/BSD-2-Clause)
[![License MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![crates.io](https://img.shields.io/crates/v/serde_dynamic_types.svg)](https://crates.io/crates/serde_dynamic_types)
[![Download numbers](https://img.shields.io/crates/d/serde_dynamic_types.svg)](https://crates.io/crates/serde_dynamic_types)
[![Travis CI](https://travis-ci.org/KizzyCode/serde_dynamic_types-rust.svg?branch=master)](https://travis-ci.org/KizzyCode/serde_dynamic_types-rust)
[![AppVeyor CI](https://ci.appveyor.com/api/projects/status/github/KizzyCode/serde_dynamic_types-rust?svg=true)](https://ci.appveyor.com/project/KizzyCode/serde-dynamic-types-rust)
[![dependency status](https://deps.rs/crate/serde_dynamic_types/0.1.0/status.svg)](https://deps.rs/crate/serde_dynamic_types/0.1.0)

# `serde_dynamic_types`
Welcome to `serde_dynamic_types` 🎉

This crate implements a simplified `serde`-compatible type model which offers a simplified representation of various
Rust data structures and allows dynamic runtime inspection. The type model can also be used as intermediate
representation for custom serializers etc.

## Why?
Since Rust is a compiled language, it makes runtime inspection and modification basically impossible. There are powerful
alternatives such as `Box<dyn Any>` or `serde`, however the first is mostly used to implement dynamic interfaces, the
latter is mostly designed to offer (de-)serialization of predefined static data structures.

`serde_dynamic_types` sits somewhere in the middle: On one side it behaves similar to `Any` since it offers a simplified
type model which can be used to build and inspect nearly arbitrarily complex data structures dynamically at runtime. On
the other side it strives to be compatible with `serde` and `serde_derive`, so that you can transform supported Rust
types and structs to `AnyValue` and vice-versa.

Furthermore it can also serve as a simplifying abstraction layer for `serde`: While `serde` is extremely versatile and
efficient, it also requires more a complex type model and implementation. If you just want to write a simple
(de-)serializer without the need for the efficiency and features `serde` offers, `serde_dynamic_types` can greatly
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
        AnyValue::Utf8String(Utf8String::new("bytes")),      // <- map key
        AnyValue::Bytes(Bytes::new(/* value of `bytes` */))  // <- associated value
    )
])
```
