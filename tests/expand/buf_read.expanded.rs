use io_enum::*;
enum Enum<A, B> {
    A(A),
    B(B),
}
impl<A, B> ::std::io::BufRead for Enum<A, B>
where
    A: ::std::io::BufRead,
    B: ::std::io::BufRead,
{
    #[inline]
    fn fill_buf(&mut self) -> ::std::io::Result<&[u8]> {
        match self {
            Enum::A(x) => <A as ::std::io::BufRead>::fill_buf(x),
            Enum::B(x) => <B as ::std::io::BufRead>::fill_buf(x),
        }
    }
    #[inline]
    fn consume(&mut self, amt: usize) {
        match self {
            Enum::A(x) => <A as ::std::io::BufRead>::consume(x, amt),
            Enum::B(x) => <B as ::std::io::BufRead>::consume(x, amt),
        }
    }
    #[inline]
    fn read_until(
        &mut self,
        byte: u8,
        buf: &mut ::std::vec::Vec<u8>,
    ) -> ::std::io::Result<usize> {
        match self {
            Enum::A(x) => <A as ::std::io::BufRead>::read_until(x, byte, buf),
            Enum::B(x) => <B as ::std::io::BufRead>::read_until(x, byte, buf),
        }
    }
    #[inline]
    fn read_line(
        &mut self,
        buf: &mut ::std::string::String,
    ) -> ::std::io::Result<usize> {
        match self {
            Enum::A(x) => <A as ::std::io::BufRead>::read_line(x, buf),
            Enum::B(x) => <B as ::std::io::BufRead>::read_line(x, buf),
        }
    }
}
fn main() {}
