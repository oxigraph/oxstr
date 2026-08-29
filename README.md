OxStr
=====

[![actions status](https://github.com/oxigraph/oxstr/workflows/build/badge.svg)](https://github.com/oxigraph/oxstr/actions)
[![Latest Version](https://img.shields.io/crates/v/oxstr.svg)](https://crates.io/crates/oxstr)
[![Released API docs](https://docs.rs/oxstr/badge.svg)](https://docs.rs/oxstr)

A Rust compact string type that can be either borrowed or reference-counted owned data.

`OxStr` is conceptually a fusion of `Arc<str>` and `Cow<'a, str>`:
it allows storing a string slice (`&str`)
or a reference-counted fixed-sized string (`Arc<str>`), enabling cheap clones of owned data.

It is not relying on an enum but uses an optimized layout, storing only a pointer and a `usize` length.
It relies on a magic bit in the length to know if the value is borrowed or owned.
If borrowed, the pointer directly targets the string bytes.
If owned, the pointer points to a memory allocation with first the reference counter, then the string bytes.

When owned, cloning is cheap and increments an atomic reference count.

```rust
use oxstr::OxStr;

let borrowed = OxStr::new("hello");
let owned = OxStr::new_owned("hello");

assert_eq!(borrowed.as_str(), "hello");
assert_eq!(owned.as_str(), "hello");
```

There is also a utility method to build a concatenated string in-place:

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
