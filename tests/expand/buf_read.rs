use io_enum::*;

#[derive(BufRead)]
enum Enum<A, B> {
    A(A),
    B(B),
}

fn main() {}
