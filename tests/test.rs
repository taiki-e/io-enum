#![cfg_attr(feature = "read_initializer", feature(read_initializer))]
#![deny(warnings)]
#![deny(unsafe_code)]
#![deny(bare_trait_objects, elided_lifetimes_in_paths)]
#![allow(dead_code)]

extern crate io_enum;

use io_enum::*;

#[derive(Read, BufRead, Write, Seek)]
enum Either<A, B> {
    A(A),
    B(B),
}
