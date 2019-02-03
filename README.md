# io-enum

[![Build Status](https://travis-ci.com/taiki-e/io-enum.svg?branch=master)](https://travis-ci.com/taiki-e/io-enum)
[![version](https://img.shields.io/crates/v/io-enum.svg)](https://crates.io/crates/io-enum/)
[![documentation](https://docs.rs/io-enum/badge.svg)](https://docs.rs/io-enum/)
[![license](https://img.shields.io/crates/l/io-enum.svg)](https://crates.io/crates/io-enum/)
[![Rustc Version](https://img.shields.io/badge/rustc-1.30+-lightgray.svg)](https://blog.rust-lang.org/2018/10/25/Rust-1.30.0.html)

\#\[derive(Read, BufRead, Write, Seek)\] for enums.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
io-enum = "0.1"
```

Now, you can use io-enum:

```rust
use io_enum::*;
```

The current version of io-enum requires Rust 1.30 or later.

## Examples

```rust
use io_enum::*;

#[derive(Read, BufRead, Write, Seek)]
enum Either<A, B> {
    A(A),
    B(B),
}

#[derive(Read, BufRead, Write, Seek)]
enum Either3<A, B, C> {
    A(A),
    B(B),
    C(C),
}
```

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
