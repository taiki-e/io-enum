# Changelog

All notable changes to this project will be documented in this file.

This project adheres to [Semantic Versioning](https://semver.org).

<!--
Note: In this file, do not use the hard wrap in the middle of a sentence for compatibility with GitHub comment style markdown rendering.
-->

## [Unreleased]

- Fix build error from dependency when built with `-Z minimal-versions`.

## [1.1.0] - 2023-03-26

- Update `syn` dependency to 2. This increase the minimum supported Rust version from Rust 1.31 to Rust 1.56.

## [1.0.2] - 2022-12-10

- Update `derive_utils` to 0.12.

## [1.0.1] - 2021-04-06

- Documentation improvements.

## [1.0.0] - 2021-01-27

No public API changes from the previous release.

## [0.2.6] - 2021-01-05

- Exclude unneeded files from crates.io.

## [0.2.5] - 2020-12-29

- Documentation improvements.

## [0.2.4] - 2020-11-06

- Update `derive_utils` to 0.11.

## [0.2.3] - 2020-06-02

- Update `derive_utils` to 0.10.

## [0.2.2] - 2019-10-08

- [Remove unstable `read_initializer` feature.](https://github.com/taiki-e/io-enum/pull/7)

## [0.2.1] - 2019-08-14

- Update `syn` and `quote` to 1.0.

- Update `derive_utils` to 0.9.

## [0.2.0] - 2019-06-16

- Transition to Rust 2018. With this change, the minimum required version will go up to Rust 1.31.

- Update to new nightly. `iovec` stabilized. This crate automatically detects the rustc version and supports `Read::read_vectored` and `Write::write_vectored` as the part of `Read` and `Write`.

- Update minimum `derive_utils` version to 0.7.2.

## [0.1.3] - 2019-03-04

- Add `iovec` feature.

- Add generated code examples.

## [0.1.2] - 2019-02-05

- Update minimum `derive_utils` version to 0.6.3.

## [0.1.1] - 2019-02-03

- Documentation improvements.

## [0.1.0] - 2019-02-03

Initial release

[Unreleased]: https://github.com/taiki-e/io-enum/compare/v1.1.0...HEAD
[1.1.0]: https://github.com/taiki-e/io-enum/compare/v1.0.2...v1.1.0
[1.0.2]: https://github.com/taiki-e/io-enum/compare/v1.0.1...v1.0.2
[1.0.1]: https://github.com/taiki-e/io-enum/compare/v1.0.0...v1.0.1
[1.0.0]: https://github.com/taiki-e/io-enum/compare/v0.2.6...v1.0.0
[0.2.6]: https://github.com/taiki-e/io-enum/compare/v0.2.5...v0.2.6
[0.2.5]: https://github.com/taiki-e/io-enum/compare/v0.2.4...v0.2.5
[0.2.4]: https://github.com/taiki-e/io-enum/compare/v0.2.3...v0.2.4
[0.2.3]: https://github.com/taiki-e/io-enum/compare/v0.2.2...v0.2.3
[0.2.2]: https://github.com/taiki-e/io-enum/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/taiki-e/io-enum/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/taiki-e/io-enum/compare/v0.1.3...v0.2.0
[0.1.3]: https://github.com/taiki-e/io-enum/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/taiki-e/io-enum/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/taiki-e/io-enum/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/taiki-e/io-enum/releases/tag/v0.1.0
