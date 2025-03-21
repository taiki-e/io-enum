// SPDX-License-Identifier: Apache-2.0 OR MIT

/*!
<!-- Note: Document from sync-markdown-to-rustdoc:start through sync-markdown-to-rustdoc:end
     is synchronized from README.md. Any changes to that range are not preserved. -->
<!-- tidy:sync-markdown-to-rustdoc:start -->

\#\[derive(Read, Write, Seek, BufRead)\] for enums.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
io-enum = "1"
```

## Examples

```
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
*/

#![doc(test(
    no_crate_inject,
    attr(
        deny(warnings, rust_2018_idioms, single_use_lifetimes),
        allow(dead_code, unused_variables)
    )
))]
#![forbid(unsafe_code)]

use derive_utils::quick_derive;
use proc_macro::TokenStream;

#[proc_macro_derive(Read)]
pub fn derive_read(input: TokenStream) -> TokenStream {
    // TODO: Add is_read_vectored once stabilized https://github.com/rust-lang/rust/issues/69941
    // TODO: Add read_buf,read_buf_exact once stabilized https://github.com/rust-lang/rust/issues/78485
    quick_derive! {
        input,
        ::std::io::Read,
        trait Read {
            #[inline]
            fn read(&mut self, buf: &mut [u8]) -> ::std::io::Result<usize>;
            #[inline]
            fn read_vectored(
                &mut self, bufs: &mut [::std::io::IoSliceMut<'_>],
            ) -> ::std::io::Result<usize>;
            #[inline]
            fn read_to_end(&mut self, buf: &mut ::std::vec::Vec<u8>) -> ::std::io::Result<usize>;
            #[inline]
            fn read_to_string(
                &mut self,
                buf: &mut ::std::string::String,
            ) -> ::std::io::Result<usize>;
            #[inline]
            fn read_exact(&mut self, buf: &mut [u8]) -> ::std::io::Result<()>;
        }
    }
}

#[proc_macro_derive(Write)]
pub fn derive_write(input: TokenStream) -> TokenStream {
    // TODO: Add is_write_vectored once stabilized https://github.com/rust-lang/rust/issues/69941
    // TODO: Add write_all_vectored once stabilized https://github.com/rust-lang/rust/issues/70436
    quick_derive! {
        input,
        ::std::io::Write,
        trait Write {
            #[inline]
            fn write(&mut self, buf: &[u8]) -> ::std::io::Result<usize>;
            #[inline]
            fn write_vectored(
                &mut self,
                bufs: &[::std::io::IoSlice<'_>],
            ) -> ::std::io::Result<usize>;
            #[inline]
            fn flush(&mut self) -> ::std::io::Result<()>;
            #[inline]
            fn write_all(&mut self, buf: &[u8]) -> ::std::io::Result<()>;
            #[inline]
            fn write_fmt(&mut self, fmt: ::std::fmt::Arguments<'_>) -> ::std::io::Result<()>;
        }
    }
}

#[proc_macro_derive(Seek)]
pub fn derive_seek(input: TokenStream) -> TokenStream {
    quick_derive! {
        input,
        ::std::io::Seek,
        trait Seek {
            #[inline]
            fn seek(&mut self, pos: ::std::io::SeekFrom) -> ::std::io::Result<u64>;
        }
    }
}

#[proc_macro_derive(BufRead)]
pub fn derive_buf_read(input: TokenStream) -> TokenStream {
    quick_derive! {
        input,
        ::std::io::BufRead,
        trait BufRead {
            #[inline]
            fn fill_buf(&mut self) -> ::std::io::Result<&[u8]>;
            #[inline]
            fn consume(&mut self, amt: usize);
            #[inline]
            fn read_until(
                &mut self, byte: u8, buf: &mut ::std::vec::Vec<u8>,
            ) -> ::std::io::Result<usize>;
            #[inline]
            fn read_line(&mut self, buf: &mut ::std::string::String) -> ::std::io::Result<usize>;
        }
    }
}
