## [0.2.0] - 2026-10-09

### Added

- `oxformat!` macro returning an `OxString`.
- `OxStr::into_owned`.
- `From` implementations to convert from and into `String`, `&str`, and `Cow<str>`.
- `no_std` support, including Serde, by disabling default features.

### Changed

- Lowered the minimum supported Rust version to 1.85.
- Reuse buffers when converting from `String` or to `String` from uniquely owned values.
- `OxStr::concat` and `OxStr::try_concat` now accept `&str` slices instead of arbitrary `AsRef<str>` elements.

### Fixed

- Memory unsoundness in concatenation with overflowing lengths or changing `AsRef<str>` results.
- Allocation failures now use Rust's allocation error handler in infallible constructors.


## [0.1.0] - 2026-08-29

### Added

- `OxStr` type and its `OxString` alias
