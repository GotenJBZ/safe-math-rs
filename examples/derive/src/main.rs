use num_traits::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub};
use safe_math::{SafeMathError, SafeMathOps, safe_math};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, SafeMathOps)]
#[SafeMathOps(add, sub, mul, div)]
struct CustomNumber {
    a: i32,
}

impl Add for CustomNumber {
    type Output = CustomNumber;
    fn add(self, rhs: Self) -> Self::Output {
        CustomNumber { a: self.a + rhs.a }
    }
}
impl Sub for CustomNumber {
    type Output = CustomNumber;
    fn sub(self, rhs: Self) -> Self::Output {
        CustomNumber { a: self.a - rhs.a }
    }
}
impl Mul for CustomNumber {
    type Output = CustomNumber;
    fn mul(self, rhs: Self) -> Self::Output {
        CustomNumber { a: self.a * rhs.a }
    }
}
impl Div for CustomNumber {
    type Output = CustomNumber;
    fn div(self, rhs: Self) -> Self::Output {
        CustomNumber { a: self.a / rhs.a }
    }
}

impl CheckedAdd for CustomNumber {
    fn checked_add(&self, rhs: &Self) -> Option<Self> {
        self.a.checked_add(rhs.a).map(|a| CustomNumber { a })
    }
}
impl CheckedSub for CustomNumber {
    fn checked_sub(&self, rhs: &Self) -> Option<Self> {
        self.a.checked_sub(rhs.a).map(|a| CustomNumber { a })
    }
}
impl CheckedMul for CustomNumber {
    fn checked_mul(&self, rhs: &Self) -> Option<Self> {
        self.a.checked_mul(rhs.a).map(|a| CustomNumber { a })
    }
}
impl CheckedDiv for CustomNumber {
    fn checked_div(&self, rhs: &Self) -> Option<Self> {
        self.a.checked_div(rhs.a).map(|a| CustomNumber { a })
    }
}

#[safe_math]
fn add(a: CustomNumber, b: CustomNumber) -> Result<CustomNumber, ()> {
    Ok(a + b)
}

#[safe_math]
fn sub(a: CustomNumber, b: CustomNumber) -> Result<CustomNumber, ()> {
    Ok(a - b)
}

#[safe_math]
fn mul(a: CustomNumber, b: CustomNumber) -> Result<CustomNumber, ()> {
    Ok(a * b)
}

#[safe_math]
fn div(a: CustomNumber, b: CustomNumber) -> Result<CustomNumber, ()> {
    Ok(a / b)
}

fn main() {
    // Test cases without overflow
    assert_eq!(
        add(CustomNumber { a: 1 }, CustomNumber { a: 2 }),
        Ok(CustomNumber { a: 3 }),
        "add(CustomNumber {{ a: 1 }}, CustomNumber {{ a: 2 }}) should be Ok(CustomNumber {{ a: 3 }})"
    );
    assert_eq!(
        sub(CustomNumber { a: 5 }, CustomNumber { a: 3 }),
        Ok(CustomNumber { a: 2 }),
        "sub(CustomNumber {{ a: 5 }}, CustomNumber {{ a: 3 }}) should be Ok(CustomNumber {{ a: 2 }})"
    );
    assert_eq!(
        mul(CustomNumber { a: 4 }, CustomNumber { a: 2 }),
        Ok(CustomNumber { a: 8 }),
        "mul(CustomNumber {{ a: 4 }}, CustomNumber {{ a: 2 }}) should be Ok(CustomNumber {{ a: 8 }})"
    );
    assert_eq!(
        div(CustomNumber { a: 6 }, CustomNumber { a: 2 }),
        Ok(CustomNumber { a: 3 }),
        "div(CustomNumber {{ a: 6 }}, CustomNumber {{ a: 2 }}) should be Ok(CustomNumber {{ a: 3 }})"
    );

    // Test cases with overflow
    assert_eq!(
        add(CustomNumber { a: i32::MAX }, CustomNumber { a: 1 }),
        Err(()),
        "add(CustomNumber {{ a: i32::MAX }}, CustomNumber {{ a: 1 }}) should be Err(())"
    );
    assert_eq!(
        sub(CustomNumber { a: i32::MIN }, CustomNumber { a: 1 }),
        Err(()),
        "sub(CustomNumber {{ a: i32::MIN }}, CustomNumber {{ a: 1 }}) should be Err(())"
    );
    assert_eq!(
        mul(CustomNumber { a: i32::MAX }, CustomNumber { a: 2 }),
        Err(()),
        "mul(CustomNumber {{ a: i32::MAX }}, CustomNumber {{ a: 2 }}) should be Err(())"
    );
    assert_eq!(
        div(CustomNumber { a: 1 }, CustomNumber { a: 0 }),
        Err(()),
        "div(CustomNumber {{ a: 1 }}, CustomNumber {{ a: 0 }}) should be Err(())"
    );
    assert_eq!(
        div(CustomNumber { a: i32::MIN }, CustomNumber { a: -1 }),
        Err(()),
        "div(CustomNumber {{ a: i32::MIN }}, CustomNumber {{ a: -1 }}) should be Err(())"
    );
}
