use io_enum::*;

#[derive(Read)]
enum Enum<A, B> {
    A(A),
    B(B),
}

fn main() {}
