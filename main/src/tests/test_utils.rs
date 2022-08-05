use std::fmt;

struct TestHelper {
    i: u32,
}

impl fmt::Display for TestHelper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Foo {}", self.i)
    }
}