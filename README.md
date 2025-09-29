# io-enum

[![crates.io](https://img.shields.io/crates/v/io-enum?style=flat-square&logo=rust)](https://crates.io/crates/io-enum)
[![docs.rs](https://img.shields.io/badge/docs.rs-io--enum-blue?style=flat-square&logo=docs.rs)](https://docs.rs/io-enum)
[![license](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue?style=flat-square)](#license)
[![msrv](https://img.shields.io/badge/msrv-1.61-blue?style=flat-square&logo=rust)](https://www.rust-lang.org)
[![github actions](https://img.shields.io/github/actions/workflow/status/taiki-e/io-enum/ci.yml?branch=main&style=flat-square&logo=github)](https://github.com/taiki-e/io-enum/actions)

<!-- tidy:sync-markdown-to-rustdoc:start:src/lib.rs -->

\#\[derive(Read, Write, Seek, BufRead)\] for enums.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
io-enum = "1"
```

## Examples

```rust
use std::{
    fs::File,
    io::{self, Write},
    path::Path,
};

use io_enum::*;

#[derive(Read, Write, Seek, BufRead)]
enum Either<A, B> {
    A(A),
    B(B),
}

fn func(path: Option<&Path>) -> impl Write {
    if let Some(path) = path {
        Either::A(File::open(path).unwrap())
    } else {
        Either::B(io::stdout())
    }
}
```

See [auto_enums] crate for how to automate patterns like this.

## Supported traits

- [`Read`](https://doc.rust-lang.org/std/io/trait.Read.html) - [example](https://github.com/taiki-e/io-enum/blob/HEAD/tests/expand/read.rs) | [generated code](https://github.com/taiki-e/io-enum/blob/HEAD/tests/expand/read.expanded.rs)
- [`BufRead`](https://doc.rust-lang.org/std/io/trait.BufRead.html) - [example](https://github.com/taiki-e/io-enum/blob/HEAD/tests/expand/buf_read.rs) | [generated code](https://github.com/taiki-e/io-enum/blob/HEAD/tests/expand/buf_read.expanded.rs)
- [`Write`](https://doc.rust-lang.org/std/io/trait.Write.html) - [example](https://github.com/taiki-e/io-enum/blob/HEAD/tests/expand/write.rs) | [generated code](https://github.com/taiki-e/io-enum/blob/HEAD/tests/expand/write.expanded.rs)
- [`Seek`](https://doc.rust-lang.org/std/io/trait.Seek.html) - [example](https://github.com/taiki-e/io-enum/blob/HEAD/tests/expand/seek.rs) | [generated code](https://github.com/taiki-e/io-enum/blob/HEAD/tests/expand/seek.expanded.rs)

## Related Projects

- [auto_enums]: A library for to allow multiple return types by automatically generated enum.
- [derive_utils]: A procedural macro helper for easily writing [derives macros][proc-macro-derive] for enums.
- [iter-enum]: \#\[derive(Iterator, DoubleEndedIterator, ExactSizeIterator, FusedIterator, Extend)\] for enums.

[auto_enums]: https://github.com/taiki-e/auto_enums
[derive_utils]: https://github.com/taiki-e/derive_utils
[iter-enum]: https://github.com/taiki-e/iter-enum
[proc-macro-derive]: https://doc.rust-lang.org/reference/procedural-macros.html#derive-macros

<!-- tidy:sync-markdown-to-rustdoc:end -->

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
