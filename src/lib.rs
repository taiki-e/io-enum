//! \#\[derive(Read, Write, Seek, BufRead)\] for enums.
//!
//! ## Examples
//!
//! ```rust
//! # #![cfg_attr(feature = "read_initializer", feature(read_initializer))]
//! use io_enum::*;
//! use std::fs::File;
//! use std::io::{self, Write};
//! use std::path::Path;
//!
//! #[derive(Read, Write, Seek, BufRead)]
//! enum Either<A, B> {
//!     A(A),
//!     B(B),
//! }
//!
//! fn foo(path: Option<&Path>) -> impl Write {
//!     if let Some(path) = path {
//!         Either::A(File::open(path).unwrap())
//!     } else {
//!         Either::B(io::stdout())
//!     }
//! }
//! ```
//!
//! See [auto_enums](https://github.com/taiki-e/auto_enums) for how to automate patterns like this.
//!
//! ## Supported traits
//!
//! * [`Read`](https://doc.rust-lang.org/std/io/trait.Read.html) - [generated code](https://github.com/taiki-e/io-enum/blob/master/doc/read.md)
//! * [`BufRead`](https://doc.rust-lang.org/std/io/trait.BufRead.html) - [generated code](https://github.com/taiki-e/io-enum/blob/master/doc/buf_read.md)
//! * [`Write`](https://doc.rust-lang.org/std/io/trait.Write.html) - [generated code](https://github.com/taiki-e/io-enum/blob/master/doc/write.md)
//! * [`Seek`](https://doc.rust-lang.org/std/io/trait.Seek.html) - [generated code](https://github.com/taiki-e/io-enum/blob/master/doc/seek.md)
//!
//! ## Crate Features
//!
//! * `read_initializer`
//!   * Disabled by default.
//!   * Implements `io::Read::read_initializer`.
//!   * This requires Rust Nightly and you need to enable the unstable [`read_initializer`](https://github.com/rust-lang/rust/issues/42788) feature gate.
//!

#![recursion_limit = "256"]
#![doc(html_root_url = "https://docs.rs/io-enum/0.2.0")]
#![doc(test(attr(deny(warnings), allow(dead_code, unused_assignments, unused_variables))))]
#![warn(unsafe_code)]
#![warn(rust_2018_idioms, unreachable_pub)]
#![warn(single_use_lifetimes)]
#![warn(clippy::all, clippy::pedantic)]

extern crate proc_macro;

use derive_utils::{derive_trait, quick_derive, EnumData as Data};
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_quote;

macro_rules! parse {
    ($input:expr) => {
        match syn::parse($input).and_then(|item| Data::from_derive(&item)) {
            Ok(data) => data,
            Err(err) => return TokenStream::from(err.to_compile_error()),
        }
    };
}

#[proc_macro_derive(Read)]
pub fn derive_read(input: TokenStream) -> TokenStream {
    #[cfg(not(stable_1_36))]
    let vectored = quote!();
    #[cfg(stable_1_36)]
    let vectored = quote! {
        #[inline]
        fn read_vectored(&mut self, bufs: &mut [::std::io::IoSliceMut<'_>]) -> ::std::io::Result<usize>;
    };

    #[cfg(not(feature = "read_initializer"))]
    let initializer = quote!();
    #[cfg(feature = "read_initializer")]
    let initializer = quote! {
        #[inline]
        unsafe fn initializer(&self) -> ::std::io::Initializer;
    };

    derive_trait!(
        parse!(input),
        parse_quote!(::std::io::Read),
        parse_quote! {
            trait Read {
                #[inline]
                fn read(&mut self, buf: &mut [u8]) -> ::std::io::Result<usize>;
                #[inline]
                fn read_to_end(&mut self, buf: &mut ::std::vec::Vec<u8>) -> ::std::io::Result<usize>;
                #[inline]
                fn read_to_string(&mut self, buf: &mut ::std::string::String) -> ::std::io::Result<usize>;
                #[inline]
                fn read_exact(&mut self, buf: &mut [u8]) -> ::std::io::Result<()>;
                #vectored
                #initializer
            }
        },
    )
    .unwrap_or_else(|e| e.to_compile_error())
    .into()
}

#[proc_macro_derive(BufRead)]
pub fn derive_buf_read(input: TokenStream) -> TokenStream {
    quick_derive! {
        input,
        (::std::io::BufRead),
        trait BufRead {
            #[inline]
            fn fill_buf(&mut self) -> ::std::io::Result<&[u8]>;
            #[inline]
            fn consume(&mut self, amt: usize);
            #[inline]
            fn read_until(&mut self, byte: u8, buf: &mut ::std::vec::Vec<u8>) -> ::std::io::Result<usize>;
            #[inline]
            fn read_line(&mut self, buf: &mut ::std::string::String) -> ::std::io::Result<usize>;
        }
    }
}

#[proc_macro_derive(Write)]
pub fn derive_write(input: TokenStream) -> TokenStream {
    #[cfg(not(stable_1_36))]
    let vectored = quote!();
    #[cfg(stable_1_36)]
    let vectored = quote! {
        #[inline]
        fn write_vectored(&mut self, bufs: &[::std::io::IoSlice<'_>]) -> ::std::io::Result<usize>;
    };

    derive_trait!(
        parse!(input),
        parse_quote!(::std::io::Write),
        parse_quote! {
            trait Write {
                #[inline]
                fn write(&mut self, buf: &[u8]) -> ::std::io::Result<usize>;
                #[inline]
                fn flush(&mut self) -> ::std::io::Result<()>;
                #[inline]
                fn write_all(&mut self, buf: &[u8]) -> ::std::io::Result<()>;
                #[inline]
                fn write_fmt(&mut self, fmt: ::std::fmt::Arguments<'_>) -> ::std::io::Result<()>;
                #vectored
            }
        },
    )
    .unwrap_or_else(|e| e.to_compile_error())
    .into()
}

#[proc_macro_derive(Seek)]
pub fn derive_seek(input: TokenStream) -> TokenStream {
    quick_derive! {
        input,
        (::std::io::Seek),
        trait Seek {
            #[inline]
            fn seek(&mut self, pos: ::std::io::SeekFrom) -> ::std::io::Result<u64>;
        }
    }
}
