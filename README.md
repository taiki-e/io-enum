# io-enum

[![crates.io](https://img.shields.io/crates/v/io-enum.svg?style=flat-square&logo=rust)](https://crates.io/crates/io-enum)
[![docs.rs](https://img.shields.io/badge/docs.rs-io--enum-blue?style=flat-square)](https://docs.rs/io-enum)
[![license](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue.svg?style=flat-square)](#license)
[![rustc](https://img.shields.io/badge/rustc-1.31+-blue.svg?style=flat-square)](https://www.rust-lang.org)
[![build status](https://img.shields.io/github/workflow/status/taiki-e/io-enum/CI/master?style=flat-square)](https://github.com/taiki-e/io-enum/actions?query=workflow%3ACI+branch%3Amaster)

\#\[derive(Read, Write, Seek, BufRead)\] for enums.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
io-enum = "0.2"
```

*Compiler support: requires rustc 1.31+*

## Examples

```rust
use io_enum::*;
use std::{
    fs::File,
    io::{self, Write},
    path::Path,
};

#[derive(Read, Write, Seek, BufRead)]
enum Either<A, B> {
    A(A),
    B(B),
}

fn foo(path: Option<&Path>) -> impl Write {
    if let Some(path) = path {
        Either::A(File::open(path).unwrap())
    } else {
        Either::B(io::stdout())
    }
}
```

See [auto_enums](https://github.com/taiki-e/auto_enums) crate for how to
automate patterns like this.

## Supported traits

- [`Read`](https://doc.rust-lang.org/std/io/trait.Read.html) - [generated code](doc/read.md)
- [`BufRead`](https://doc.rust-lang.org/std/io/trait.BufRead.html) - [generated code](doc/buf_read.md)
- [`Write`](https://doc.rust-lang.org/std/io/trait.Write.html) - [generated code](doc/write.md)
- [`Seek`](https://doc.rust-lang.org/std/io/trait.Seek.html) - [generated code](doc/seek.md)

## Related Projects

- [auto_enums]: A library for to allow multiple return types by automatically generated enum.
- [derive_utils]: A procedural macro helper for easily writing [derives macros][proc-macro-derive] for enums.
- [futures-enum]: \#\[derive(Future, Stream, Sink, AsyncRead, AsyncWrite, AsyncSeek, AsyncBufRead)\] for enums.
- [iter-enum]: \#\[derive(Iterator, DoubleEndedIterator, ExactSizeIterator, Extend)\] for enums.

[auto_enums]: https://github.com/taiki-e/auto_enums
[derive_utils]: https://github.com/taiki-e/derive_utils
[futures-enum]: https://github.com/taiki-e/futures-enum
[iter-enum]: https://github.com/taiki-e/iter-enum

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
