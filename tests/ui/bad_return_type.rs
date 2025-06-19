use safe_math_macros::safe_math;

#[safe_math]
fn wrong_return_type() -> i32 { // should return Result<_, _>
    1 + 2
}

fn main() {} 