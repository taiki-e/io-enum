## [`Read`](https://doc.rust-lang.org/std/io/trait.Read.html)

When deriving for enum like the following:

```rust
#[derive(Read)]
enum Enum<A, B> {
    A(A),
    B(B),
}
```

Code like this will be generated:

```rust
enum Enum<A, B> {
    A(A),
    B(B),
}

impl<A, B> ::std::io::Read for Enum<A, B>
where
    A: ::std::io::Read,
    B: ::std::io::Read,
{
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> ::std::io::Result<usize> {
        match self {
            Enum::A(x) => ::std::io::Read::read(x, buf),
            Enum::B(x) => ::std::io::Read::read(x, buf),
        }
    }

    #[inline]
    fn read_to_end(&mut self, buf: &mut ::std::vec::Vec<u8>) -> ::std::io::Result<usize> {
        match self {
            Enum::A(x) => ::std::io::Read::read_to_end(x, buf),
            Enum::B(x) => ::std::io::Read::read_to_end(x, buf),
        }
    }

    #[inline]
    fn read_to_string(&mut self, buf: &mut ::std::string::String) -> ::std::io::Result<usize> {
        match self {
            Enum::A(x) => ::std::io::Read::read_to_string(x, buf),
            Enum::B(x) => ::std::io::Read::read_to_string(x, buf),
        }
    }

    #[inline]
    fn read_exact(&mut self, buf: &mut [u8]) -> ::std::io::Result<()> {
        match self {
            Enum::A(x) => ::std::io::Read::read_exact(x, buf),
            Enum::B(x) => ::std::io::Read::read_exact(x, buf),
        }
    }
}
```
