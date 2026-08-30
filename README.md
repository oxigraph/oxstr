OxStr
=====

[![actions status](https://github.com/oxigraph/oxstr/workflows/build/badge.svg)](https://github.com/oxigraph/oxstr/actions)
[![Latest Version](https://img.shields.io/crates/v/oxstr.svg)](https://crates.io/crates/oxstr)
[![Released API docs](https://docs.rs/oxstr/badge.svg)](https://docs.rs/oxstr)

A compact Rust string type that can hold either borrowed or reference-counted owned data.

`OxStr<'a>` is conceptually a fusion of `Arc<str>` and `Cow<'a, str>`:
it can store either a borrowed string slice (`&'a str`) without allocating or an
immutable-length, reference-counted string similar to `Arc<str>`. Cloning owned values is cheap.

It is designed for use cases where a string is created once from a known input and then reused,
cloned, and shared extensively. The `OxString` type alias is available when a fully owned or
`'static` value is required.

Rather than using an enum, `OxStr` has a compact two-word layout containing a pointer and a
`usize` length. The most significant bit of the length identifies whether the value is borrowed
or owned. For a borrowed value, the pointer targets the string bytes directly. For an owned value,
the pointer targets an allocation containing an atomic reference count followed by the string bytes.

Cloning an owned value increments its atomic reference count; cloning a borrowed value remains
borrowed and does not allocate.

```rust
use oxstr::OxStr;

let borrowed = OxStr::new("hello");
let owned = OxStr::new_owned("hello");

assert_eq!(borrowed.as_str(), "hello");
assert_eq!(owned.as_str(), "hello");
```

The `concat` method allocates one owned value and copies the supplied string slices directly into it:

```rust
use oxstr::OxStr;

assert_eq!(OxStr::concat(["foo", " ", "bar"]), "foo bar");
```


## License

This project is licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
  `<http://www.apache.org/licenses/LICENSE-2.0>`)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or
  `<http://opensource.org/licenses/MIT>`)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in oxstr by
you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
