# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Next

## [0.8.2] - 2020-05-06
### Changes
- Remove unneeded files from crate (#[41](https://github.com/contain-rs/vec-map/pull/41))

## [0.8.1] - 2018-05-11
### Additions
- Introduce `serde` feature to enable serde serialization (#[40](https://github.com/contain-rs/vec-map/pull/40))
- Add `values_mut` and `shrink_to_fit` methods (#[37](https://github.com/contain-rs/vec-map/pull/37))
- Add `retain` method (#[35](https://github.com/contain-rs/vec-map/pull/35))
### Fixes
- Fix and Rust 1.26 warnings (#[39](https://github.com/contain-rs/vec-map/pull/39))

## [0.8.0] - 2017-05-08
### Fixes
- Fix broken serde test (#[34](https://github.com/contain-rs/vec-map/pull/34))
### Additions
- Track number of elements for more efficient `len` (#[33](https://github.com/contain-rs/vec-map/pull/33))
- Add `clone_from` impl for `VecMap` (#[30](https://github.com/contain-rs/vec-map/pull/30))
### Changes
- Update `serde` dependency to 1.0

## [0.7.0] - 2017-02-27
### Changes
- Update `serde` dependency from 0.6 to 0.9 (#[17](https://github.com/contain-rs/vec-map/pull/17), #[25](https://github.com/contain-rs/vec-map/pull/25))

## [0.6.0] - 2016-02-15
### Fixes
- Add `Drain` documentation
- Make `IntoIter`, `Keys`, and `Values` covariant.
- Remove unnecesary `nightly` feature.
### Additions
- Adding nightly serde support for VecMap under "eders" feature flag (#[15](https://github.com/contain-rs/vec-map/pull/15))
### Changes
- Change VecMap interface to use `usize` not `&usize` (API change).

## [0.4.0] - 2015-11-08
### Changes
- Use Rust 1.5.0 iterator comparison API. (#[8](https://github.com/contain-rs/vec-map/pull/8))
- Use `debug_map` builder on stable.
- replace uses of removed `std::hash::hash` in tests. (#[6](https://github.com/contain-rs/vec-map/pull/6))

## [0.3.0] - 2015-07-11
### Additions
- derive `Clone` impl
- use debug builder for formatting on nightly
### Fixes
- remove unnecessary allocation from test

## [0.2.0] - 2015-07-10
### Additions
- Build on stable and nightly

## [0.1.0] - 2015-07-09
- Initial release