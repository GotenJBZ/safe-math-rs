use safe_math::safe_math;
use std::sync::atomic::{AtomicUsize, Ordering};

/// Test that compound assignments don't cause side effects by evaluating
/// the left-hand side expression multiple times
#[test]
fn test_compound_assignment_no_side_effects() {
    static CALL_COUNT: AtomicUsize = AtomicUsize::new(0);

    // Function that has side effects - increments a counter each time it's called
    fn get_index() -> usize {
        let result = CALL_COUNT.load(Ordering::SeqCst);
        CALL_COUNT.store(result + 1, Ordering::SeqCst);
        result
    }

    #[safe_math]
    fn test_add_assign() -> Result<u8, ()> {
        let mut arr = [10u8, 20u8, 30u8];
        // This should only call get_index() once, not twice
        arr[get_index()] += 5;
        Ok(arr[0])
    }

    CALL_COUNT.store(0, Ordering::SeqCst);
    let result = test_add_assign();

    // Verify the result is correct
    assert_eq!(result, Ok(15));

    // Verify get_index was called exactly once
    let call_count = CALL_COUNT.load(Ordering::SeqCst);
    assert_eq!(
        call_count, 1,
        "get_index should be called exactly once, but was called {call_count} times"
    );
}

#[test]
fn test_all_compound_assignments() {
    #[safe_math]
    fn test_compound_ops() -> Result<(u8, u8, u8, u8, u8), ()> {
        let mut a = 10u8;
        let mut b = 20u8;
        let mut c = 6u8;
        let mut d = 15u8;
        let mut e = 17u8;

        a += 5; // 10 + 5 = 15
        b -= 5; // 20 - 5 = 15
        c *= 2; // 6 * 2 = 12
        d /= 3; // 15 / 3 = 5
        e %= 7; // 17 % 7 = 3

        Ok((a, b, c, d, e))
    }

    assert_eq!(test_compound_ops(), Ok((15, 15, 12, 5, 3)));
}

#[test]
fn test_compound_assignment_overflow() {
    #[safe_math]
    fn test_overflow() -> Result<u8, ()> {
        let mut x = 255u8;
        x += 1; // This should overflow and return an error
        Ok(x)
    }

    assert!(test_overflow().is_err());
}

#[test]
fn test_compound_assignment_underflow() {
    #[safe_math]
    fn test_underflow() -> Result<u8, ()> {
        let mut x = 0u8;
        x -= 1; // This should underflow and return an error
        Ok(x)
    }

    assert!(test_underflow().is_err());
}
