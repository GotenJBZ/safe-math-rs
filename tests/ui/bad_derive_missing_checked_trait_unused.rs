use safe_math::SafeMathOps;
use std::ops::Add;

#[derive(Debug,Copy,Clone,SafeMathOps)]
#[SafeMathOps(add)]
struct Foo(i32);

impl Add for Foo {
    type Output = Foo;
    fn add(self, other: Foo) -> Foo {
        Foo(self.0 + other.0)
    }
}

fn main() {} 