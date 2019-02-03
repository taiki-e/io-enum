//! \#\[derive(Read, BufRead, Write, Seek)\] for enums.
//!
//! ## Examples
//!
//! ```rust
//! # #![cfg_attr(feature = "read_initializer", feature(read_initializer))]
//! # extern crate io_enum;
//! use io_enum::*;
//!
//! #[derive(Read, BufRead, Write, Seek)]
//! enum Either<A, B> {
//!     A(A),
//!     B(B),
//! }
//!
//! #[derive(Read, BufRead, Write, Seek)]
//! enum Either3<A, B, C> {
//!     A(A),
//!     B(B),
//!     C(C),
//! }
//! ```
//!

#![crate_type = "proc-macro"]
#![recursion_limit = "256"]
#![doc(html_root_url = "https://docs.rs/io-enum/0.1.0")]
#![deny(bare_trait_objects, elided_lifetimes_in_paths)]

extern crate derive_utils;
extern crate proc_macro;

use derive_utils::quick_derive;
use proc_macro::TokenStream;

#[cfg(not(feature = "read_initializer"))]
#[proc_macro_derive(Read)]
pub fn derive_read(input: TokenStream) -> TokenStream {
    quick_derive! {
        input,
        (::std::io::Read),
        trait Read {
            #[inline]
            fn read(&mut self, buf: &mut [u8]) -> ::std::io::Result<usize>;
            #[inline]
            fn read_to_end(&mut self, buf: &mut ::std::vec::Vec<u8>) -> ::std::io::Result<usize>;
            #[inline]
            fn read_to_string(&mut self, buf: &mut ::std::string::String) -> ::std::io::Result<usize>;
            #[inline]
            fn read_exact(&mut self, buf: &mut [u8]) -> ::std::io::Result<()>;
        }
    }
}

#[cfg(feature = "read_initializer")]
#[proc_macro_derive(Read)]
pub fn derive_read(input: TokenStream) -> TokenStream {
    quick_derive! {
        input,
        (::std::io::Read),
        trait Read {
            #[inline]
            fn read(&mut self, buf: &mut [u8]) -> ::std::io::Result<usize>;
            #[inline]
            fn read_to_end(&mut self, buf: &mut ::std::vec::Vec<u8>) -> ::std::io::Result<usize>;
            #[inline]
            fn read_to_string(&mut self, buf: &mut ::std::string::String) -> ::std::io::Result<usize>;
            #[inline]
            fn read_exact(&mut self, buf: &mut [u8]) -> ::std::io::Result<()>;
            #[inline]
            unsafe fn initializer(&self) -> ::std::io::Initializer;
        }
    }
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
    quick_derive! {
        input,
        (::std::io::Write),
        trait Write {
            #[inline]
            fn write(&mut self, buf: &[u8]) -> ::std::io::Result<usize>;
            #[inline]
            fn flush(&mut self) -> ::std::io::Result<()>;
            #[inline]
            fn write_all(&mut self, buf: &[u8]) -> ::std::io::Result<()>;
            #[inline]
            fn write_fmt(&mut self, fmt: ::std::fmt::Arguments) -> ::std::io::Result<()>;
        }
    }
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
