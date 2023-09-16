// SPDX-License-Identifier: Apache-2.0 OR MIT

use io_enum::*;

#[derive(Write)]
enum Enum<A, B> {
    A(A),
    B(B),
}

fn main() {}
