use std::fmt;

struct Config {
    i: u32,
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Foo {}", self.i)
    }
}