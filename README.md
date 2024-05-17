# saturate &emsp; [![Test Status]][actions]&thinsp;[![Crate Version]][crates]&thinsp;[![Rust Version]][crates]

[test status]: https://img.shields.io/github/actions/workflow/status/staticintlucas/saturate/test.yml?branch=main&label=tests&style=flat-square
[crate version]: https://img.shields.io/crates/v/saturate?style=flat-square
[rust version]: https://img.shields.io/badge/rust-1.45%2B-informational?style=flat-square

[actions]: https://github.com/staticintlucas/saturate/actions?query=branch%3Amain
[crates]: https://crates.io/crates/saturate

This crate provides a set of traits for saturating conversion between different numeric types.

The trait `SaturatingFrom` is implemented by default for all standard numeric types.
A blanket implementation of `SaturatingInto` is also provided,
mirroring the standard library's `From` and `Into` traits.

## Example

```Rust
use saturate::{SaturatingFrom, SaturatingInto};

assert_eq!(0, u8::saturating_from(-26));
assert_eq!(u32::MAX, i64::MAX.saturating_into());
assert!(f32::saturating_from(u128::MAX).is_infinite()); // out of range => infinity
assert_eq!(u8::MAX, 300.0.saturating_into());
```

## Licence

Licensed under either of

* Apache License, Version 2.0 ([LICENCE-APACHE](LICENCE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0][apache-licence])
* MIT license ([LICENCE-MIT](LICENCE-MIT) or [http://opensource.org/licenses/MIT][mit-licence])

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without
any additional terms or conditions.

[apache-licence]: http://www.apache.org/licenses/LICENSE-2.0
[mit-licence]: http://opensource.org/licenses/MIT
