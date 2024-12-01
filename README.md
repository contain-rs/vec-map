<div align="center">
  <h1>vec-map</h1>
  <p>
    <strong>A compact vector of bits.</strong>
  </p>
  <p>

[![crates.io][crates.io shield]][crates.io link]
[![Documentation][docs.rs badge]][docs.rs link]
![Rust CI][github ci badge]
[![rustc 1.63+]][Rust 1.63]
<br />
<br />
[![Dependency Status][deps.rs status]][deps.rs link]
[![Download Status][shields.io download count]][crates.io link]

  </p>
</div>

[crates.io shield]: https://img.shields.io/crates/v/vec-map?label=latest
[crates.io link]: https://crates.io/crates/vec-map
[docs.rs badge]: https://docs.rs/vec-map/badge.svg?version=0.8.2
[docs.rs link]: https://docs.rs/vec-map/0.8.2/vec_map/
[github ci badge]: https://github.com/contain-rs/vec-map/workflows/Rust/badge.svg?branch=master
[rustc 1.63+]: https://img.shields.io/badge/rustc-1.63%2B-blue.svg
[deps.rs status]: https://deps.rs/crate/vec-map/0.8.2/status.svg
[deps.rs link]: https://deps.rs/crate/vec-map/0.8.2
[shields.io download count]: https://img.shields.io/crates/d/vec-map.svg

## Usage

Add this to your Cargo.toml:

```toml
[dependencies]
vec-map = "0.8"
```

Since Rust 2018, `extern crate` is no longer mandatory. If your edition is old (Rust 2015),
add this to your crate root:

```rust
extern crate vec_map;
```

If you want [serde](https://github.com/serde-rs/serde) support, include the feature like this:

```toml
[dependencies]
vec-map = { version = "0.8", features = ["serde"] }
```

If you want to use vec-map in a program that has `#![no_std]`, just drop default features:

```toml
[dependencies]
vec-map = { version = "0.8", default-features = false }
```

If you want to use serde with the alloc crate instead of std, just use the `serde_no_std` feature:

```toml
[dependencies]
vec-map = { version = "0.8", default-features = false, features = ["serde", "serde_no_std"] }
```

<!-- cargo-rdme start -->

### Description

A simple map based on a vector for small integer keys. Space requirements
are **O(highest integer key)**.

<!-- cargo-rdme end -->

## License

Dual-licensed for compatibility with the Rust project.

Licensed under the Apache License Version 2.0: http://www.apache.org/licenses/LICENSE-2.0,
or the MIT license: http://opensource.org/licenses/MIT, at your option.
