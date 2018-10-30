# chip-8-rs

[![NPM version](https://img.shields.io/npm/v/chip-8-wasm.svg?style=flat)](https://www.npmjs.com/package/chip-8-wasm)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Build Status](https://travis-ci.org/jeffrey-xiao/chip-8-rs.svg?branch=master)](https://travis-ci.org/jeffrey-xiao/chip-8-rs)
[![codecov](https://codecov.io/gh/jeffrey-xiao/extended-collections-rs/branch/master/graph/badge.svg)](https://codecov.io/gh/jeffrey-xiao/extended-collections-rs)

A CHIP-8/SCHIP emulator written in Rust that compiles to WebAssembly.

## Documentation

See the [documentation](https://jeffreyxiao.me/chip-8-rs) for more information about using
`chip-8-rs`.

## Usage

Install `chip-8-wasm` from [npm](https://www.npmjs.com/):
```
$ npm install chip-8-wasm
```

Example usage: [`chip-8-web`](https://gitlab.com/jeffrey-xiao/chip-8-web).

## Changelog

### [1.0.0] - 2018-10-06

 - Minor clippy fixes.
 - Initial stable release.

### [0.2.0] - 2018-09-28

 - Initial functional release.

## References

 - [Cowgod's Chip-8 Technical Reference v1.0](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)
 - [Erik Bryntse's SUPER-CHIP v1.1](http://devernay.free.fr/hacks/chip8/schip.txt)

## License

`chip-8-rs` is distributed under the terms of both the MIT License and the Apache License (Version
2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for more details.
