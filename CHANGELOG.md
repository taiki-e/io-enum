# Changelog

All notable changes to this project will be documented in this file.

This project adheres to [Semantic Versioning](https://semver.org).

## [Unreleased]

## [0.2.2] - 2019-10-08

* [Removed unstable `"read_initializer"` feature.][7]

[7]: https://github.com/taiki-e/io-enum/pull/7

## [0.2.1] - 2019-08-14

* Updated `syn` and `quote` to 1.0.

* Updated `derive_utils` to 0.9.

## [0.2.0] - 2019-06-16

* Transition to Rust 2018. With this change, the minimum required version will go up to Rust 1.31.

* Updated to new nightly. `iovec` stabilized. This crate automatically detects the rustc version and supports `Read::read_vectored` and `Write::write_vectored` as the part of `Read` and `Write`.

* Updated minimum `derive_utils` version to 0.7.2.

## [0.1.3] - 2019-03-04

* Add `"iovec"` feature.

* Add generated code examples.

## [0.1.2] - 2019-02-05

* Update minimum `derive_utils` version to 0.6.3.

## [0.1.1] - 2019-02-03

* Improve documentations.

## [0.1.0] - 2019-02-03

Initial release

[Unreleased]: https://github.com/taiki-e/io-enum/compare/v0.2.2...HEAD
[0.2.2]: https://github.com/taiki-e/io-enum/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/taiki-e/io-enum/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/taiki-e/io-enum/compare/v0.1.3...v0.2.0
[0.1.3]: https://github.com/taiki-e/io-enum/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/taiki-e/io-enum/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/taiki-e/io-enum/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/taiki-e/io-enum/releases/tag/v0.1.0
