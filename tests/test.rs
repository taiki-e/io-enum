#![warn(unsafe_code)]
#![warn(rust_2018_idioms)]
#![allow(dead_code)]

use io_enum::*;

#[derive(Read, Write, Seek, BufRead)]
enum Either<A, B> {
    A(A),
    B(B),
}
