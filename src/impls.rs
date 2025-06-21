use crate::error::SafeMathError;
use crate::ops::{SafeAdd, SafeDiv, SafeMathOps, SafeMul, SafeRem, SafeSub};
use sealed::{IsSafeAdd, IsSafeDiv, IsSafeMul, IsSafeRem, IsSafeSub};

macro_rules! impl_safe_math_ops {
    ($($op:ident, $trait:ident),*) => {
        $(
            #[inline(always)]
            pub fn $op<T: $trait>(a: T, b: T) -> Result<T, SafeMathError> {
                a.$op(b)
            }
        )*
    };
}

impl_safe_math_ops!(
    safe_add, SafeAdd, safe_sub, SafeSub, safe_mul, SafeMul, safe_div, SafeDiv, safe_rem, SafeRem
);

macro_rules! impl_safe_ops {
    (
        $(
            ($trait_name:ident, $trait_name_str:ident, $method_name:ident, $checked_method:ident, $bound:ident, $err:expr)
        ),* $(,)?
    ) => {
        $(
            #[diagnostic::do_not_recommend]
            impl<T> $trait_name for T
            where
                T: $bound + std::ops::$trait_name_str<Output = T> + Copy,
            {
                #[inline(always)]
                fn $method_name(self, rhs: T) -> Result<T, SafeMathError> {
                    self.$checked_method(&rhs).ok_or($err)
                }
            }
        )*
    };
}

impl_safe_ops!(
    (
        SafeAdd,
        Add,
        safe_add,
        checked_add,
        IsSafeAdd,
        SafeMathError::Overflow
    ),
    (
        SafeSub,
        Sub,
        safe_sub,
        checked_sub,
        IsSafeSub,
        SafeMathError::Overflow
    ),
    (
        SafeMul,
        Mul,
        safe_mul,
        checked_mul,
        IsSafeMul,
        SafeMathError::Overflow
    ),
    (
        SafeDiv,
        Div,
        safe_div,
        checked_div,
        IsSafeDiv,
        SafeMathError::DivisionByZero
    ),
    (
        SafeRem,
        Rem,
        safe_rem,
        checked_rem,
        IsSafeRem,
        SafeMathError::DivisionByZero
    ),
);

macro_rules! impl_safe_float_ops {
    ($($trait:ident, $method:ident, $op:tt),*) => {
        $(
            #[diagnostic::do_not_recommend]
            impl $trait for f32 {
                #[inline(always)]
                fn $method(self, rhs: Self) -> Result<Self, SafeMathError> {
                    let res = self $op rhs;
                    res.is_finite().then(|| res).ok_or(SafeMathError::NonFinite)
                }
            }

            #[diagnostic::do_not_recommend]
            impl $trait for f64 {
                #[inline(always)]
                fn $method(self, rhs: Self) -> Result<Self, SafeMathError> {
                    let res = self $op rhs;
                    res.is_finite().then(|| res).ok_or(SafeMathError::NonFinite)
                }
            }
        )*
    };
}

impl_safe_float_ops!(
    SafeAdd, safe_add, +,
    SafeSub, safe_sub, -,
    SafeMul, safe_mul, *,
    SafeDiv, safe_div, /,
    SafeRem, safe_rem, %
);

#[diagnostic::do_not_recommend]

impl<T> SafeMathOps for T
where
    T: SafeAdd + SafeSub + SafeMul + SafeDiv + SafeRem + Copy,
{
    #[inline(always)]
    fn safe_add(self, rhs: Self) -> Result<Self, SafeMathError> {
        <Self as SafeAdd>::safe_add(self, rhs)
    }
    #[inline(always)]
    fn safe_sub(self, rhs: Self) -> Result<Self, SafeMathError> {
        <Self as SafeSub>::safe_sub(self, rhs)
    }
    #[inline(always)]
    fn safe_mul(self, rhs: Self) -> Result<Self, SafeMathError> {
        <Self as SafeMul>::safe_mul(self, rhs)
    }
    #[inline(always)]
    fn safe_div(self, rhs: Self) -> Result<Self, SafeMathError> {
        <Self as SafeDiv>::safe_div(self, rhs)
    }
    #[inline(always)]
    fn safe_rem(self, rhs: Self) -> Result<Self, SafeMathError> {
        <Self as SafeRem>::safe_rem(self, rhs)
    }
}

mod sealed {
    use num_traits::ops::checked::{CheckedAdd, CheckedDiv, CheckedMul, CheckedRem, CheckedSub};
    pub trait Sealed {}

    macro_rules! impl_sealed {
        ($($t:ty),*) => {
            $(
                impl Sealed for $t {}
            )*
        };
    }

    impl_sealed!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

    pub trait IsSafeAdd: Sealed + CheckedAdd {}
    pub trait IsSafeSub: Sealed + CheckedSub {}
    pub trait IsSafeMul: Sealed + CheckedMul {}
    pub trait IsSafeDiv: Sealed + CheckedDiv {}
    pub trait IsSafeRem: Sealed + CheckedRem {}

    impl<T> IsSafeAdd for T where T: Sealed + CheckedAdd {}
    impl<T> IsSafeSub for T where T: Sealed + CheckedSub {}
    impl<T> IsSafeMul for T where T: Sealed + CheckedMul {}
    impl<T> IsSafeDiv for T where T: Sealed + CheckedDiv {}
    impl<T> IsSafeRem for T where T: Sealed + CheckedRem {}
}
