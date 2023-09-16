// SPDX-License-Identifier: Apache-2.0 OR MIT

#![warn(rust_2018_idioms, single_use_lifetimes)]
#![allow(dead_code)]

use std::io;

use io_enum::{BufRead, Read, Seek, Write};

#[derive(Read, Write, Seek, BufRead)]
enum Either<A, B> {
    A(A),
    B(B),
}

fn _assert_impl<T: io::Read + io::Write + io::Seek + io::BufRead>() {
    fn __assert_impl<T: io::Read + io::Write + io::Seek + io::BufRead>() {}
    __assert_impl::<Either<T, T>>();
}
