use safe_math_macros::SafeMathOps;

// Duplicate operation "add"
#[derive(SafeMathOps)]
#[SafeMathOps(add, add)]
struct Foo(i32);

// Unknown operation "unknown"
#[derive(SafeMathOps)]
#[SafeMathOps(unknown)]
struct Bar(i32);

// Missing list entirely
#[derive(SafeMathOps)]
struct Baz(i32);

fn main() {} 