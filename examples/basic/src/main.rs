use safe_math_rs::safe_math;

#[safe_math]
fn add(a: u8, b: u8) -> Result<u8, ()> {
    Ok(a + b)
}

#[safe_math]
fn sub(a: u8, b: u8) -> Result<u8, ()> {
    Ok(a - b)
}

#[safe_math]
fn mul(a: u8, b: u8) -> Result<u8, ()> {
    Ok(a * b)
}

#[safe_math]
fn div(a: u8, b: u8) -> Result<u8, ()> {
    Ok(a / b)
}

fn main() {
    assert_eq!(add(10, 20), Ok(30)); // Good case
    assert!(add(255, 1).is_err()); // Bad case - overflow

    assert_eq!(sub(30, 10), Ok(20)); // Good case
    assert!(sub(10, 20).is_err()); // Bad case - underflow

    assert_eq!(mul(10, 20), Ok(200)); // Good case
    assert!(mul(255, 2).is_err()); // Bad case - overflow

    assert_eq!(div(30, 6), Ok(5)); // Good case
    assert!(div(10, 0).is_err()); // Bad case - division by zero
}
