# chipo-rs

[![NPM version](https://img.shields.io/npm/v/chipo.svg?style=flat)](https://www.npmjs.com/package/chipo)
[![chipo](http://meritbadge.herokuapp.com/chipo)](https://crates.io/crates/chipo)
[![Documentation](https://docs.rs/chipo/badge.svg)](https://docs.rs/chipo)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Build Status](https://travis-ci.org/jeffrey-xiao/chipo-rs.svg?branch=master)](https://travis-ci.org/jeffrey-xiao/chipo-rs)
[![codecov](https://codecov.io/gh/jeffrey-xiao/chipo-rs/branch/master/graph/badge.svg)](https://codecov.io/gh/jeffrey-xiao/chipo-rs)

CHIP-8 Oxidized is a CHIP-8/SCHIP emulator written in Rust that can compile to WebAssembly.

## JavaScript Usage

Install `chipo` from [npm](https://www.npmjs.com/):

```text
$ npm install chipo
```

Example JavaScript usage: [`chipo-web`](https://gitlab.com/jeffrey-xiao/chipo-web).

## Rust Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
chipo = "*"
```

and this to your crate root if you are using Rust 2015:

```rust
extern crate chipo;
```

## Changelog

See [CHANGELOG](CHANGELOG.md) for more details.

## References

- [Cowgod's Chip-8 Technical Reference v1.0](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)
- [Erik Bryntse's SUPER-CHIP v1.1](http://devernay.free.fr/hacks/chip8/schip.txt)

## License

`chipo-rs` is distributed under the terms of both the MIT License and the Apache License (Version
2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for more details.
