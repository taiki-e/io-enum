# io-enum

[![Build Status][azure-badge]][azure-url]
[![Crates.io][crates-version-badge]][crates-url]
[![Docs.rs][docs-badge]][docs-url]
[![License][crates-license-badge]][crates-url]
[![Minimum supported Rust version][rustc-badge]][rustc-url]

[azure-badge]: https://dev.azure.com/taiki-e/taiki-e/_apis/build/status/taiki-e.io-enum?branchName=master
[azure-url]: https://dev.azure.com/taiki-e/taiki-e/_build/latest?definitionId=8&branchName=master
[crates-version-badge]: https://img.shields.io/crates/v/io-enum.svg
[crates-license-badge]: https://img.shields.io/crates/l/io-enum.svg
[crates-badge]: https://img.shields.io/crates/v/io-enum.svg
[crates-url]: https://crates.io/crates/io-enum/
[docs-badge]: https://docs.rs/io-enum/badge.svg
[docs-url]: https://docs.rs/io-enum/
[rustc-badge]: https://img.shields.io/badge/rustc-1.31+-lightgray.svg
[rustc-url]: https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html

\#\[derive(Read, Write, Seek, BufRead)\] for enums.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
io-enum = "0.2"
```

The current io-enum requires Rust 1.31 or later.

## Examples

```rust
use io_enum::*;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

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

See [auto_enums](https://github.com/taiki-e/auto_enums) for how to automate patterns like this.

## Supported traits

* [`Read`](https://doc.rust-lang.org/std/io/trait.Read.html) - [generated code](doc/read.md)
* [`BufRead`](https://doc.rust-lang.org/std/io/trait.BufRead.html) - [generated code](doc/buf_read.md)
* [`Write`](https://doc.rust-lang.org/std/io/trait.Write.html) - [generated code](doc/write.md)
* [`Seek`](https://doc.rust-lang.org/std/io/trait.Seek.html) - [generated code](doc/seek.md)

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
