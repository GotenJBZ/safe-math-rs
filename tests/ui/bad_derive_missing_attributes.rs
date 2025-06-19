use safe_math::{safe_math, SafeMathOps};
use std::ops::Sub;
use num_traits::CheckedSub;

#[derive(Debug,Copy,Clone,SafeMathOps)]
#[SafeMathOps(sub)]
struct Foo(i32);

impl Sub for Foo {
    type Output = Foo;
    fn sub(self, other: Foo) -> Foo {
        Foo(self.0 - other.0)
    }
}

impl CheckedSub for Foo {
    fn checked_sub(&self, other: &Foo) -> Option<Foo> {
        self.0.checked_sub(other.0).map(Foo)
    }
}


#[safe_math]
fn test_add(a: Foo, b: Foo) -> Result<Foo, ()> {
    Ok(a + b)
}

fn main() {} 