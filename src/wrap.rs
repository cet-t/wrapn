use std::{
    fmt::{self, Display, Formatter},
    ops::*,
    num::Wrapping,
};

/// Trait for types that support wrapping arithmetic operations.
///
/// This trait is implemented for all primitive integer types.
pub trait Num: Copy + Sized {
    fn wrapping_add(self, rhs: Self) -> Self;
    fn wrapping_sub(self, rhs: Self) -> Self;
    fn wrapping_mul(self, rhs: Self) -> Self;
    fn wrapping_div(self, rhs: Self) -> Self;
    fn wrapping_rem(self, rhs: Self) -> Self;
    fn wrapping_neg(self) -> Self;
}

macro_rules! num_impl {
    ($($t:ty),* $(,)?) => {
        $(impl Num for $t {
            fn wrapping_add(self, rhs: Self) -> Self { self.wrapping_add(rhs) }
            fn wrapping_sub(self, rhs: Self) -> Self { self.wrapping_sub(rhs) }
            fn wrapping_mul(self, rhs: Self) -> Self { self.wrapping_mul(rhs) }
            fn wrapping_div(self, rhs: Self) -> Self { self.wrapping_div(rhs) }
            fn wrapping_rem(self, rhs: Self) -> Self { self.wrapping_rem(rhs) }
            fn wrapping_neg(self) -> Self { self.wrapping_neg() }
        })*
    };
}

num_impl! {
    u8, u16, u32, u64, u128, usize,
    i8, i16, i32, i64, i128, isize,
}

macro_rules! wrap_binary {
    ($trait:ident, $method:ident, $wrapping:ident) => {
        impl<T: Copy + Num> $trait<Wrap<T>> for Wrap<T> {
            type Output = Wrap<T>;
            fn $method(self, rhs: Wrap<T>) -> Self::Output {
                Wrap(Wrapping(self.0 .0.$wrapping(rhs.0 .0)))
            }
        }
        impl<T: Copy + Num> $trait<T> for Wrap<T> {
            type Output = Wrap<T>;
            fn $method(self, rhs: T) -> Self::Output {
                Wrap(Wrapping(self.0 .0.$wrapping(rhs)))
            }
        }
    };
}

wrap_binary!(Add, add, wrapping_add);
wrap_binary!(Sub, sub, wrapping_sub);
wrap_binary!(Mul, mul, wrapping_mul);
wrap_binary!(Div, div, wrapping_div);
wrap_binary!(Rem, rem, wrapping_rem);

macro_rules! wrap_binary_assign {
    ($trait:ident, $method:ident, $wrapping:ident) => {
        impl<T: Copy + Num> $trait<Wrap<T>> for Wrap<T> {
            fn $method(&mut self, rhs: Wrap<T>) {
                self.0 = Wrapping(self.0 .0.$wrapping(rhs.0 .0));
            }
        }
        impl<T: Copy + Num> $trait<T> for Wrap<T> {
            fn $method(&mut self, rhs: T) {
                self.0 = Wrapping(self.0 .0.$wrapping(rhs));
            }
        }
    };
}

wrap_binary_assign!(AddAssign, add_assign, wrapping_add);
wrap_binary_assign!(SubAssign, sub_assign, wrapping_sub);
wrap_binary_assign!(MulAssign, mul_assign, wrapping_mul);
wrap_binary_assign!(DivAssign, div_assign, wrapping_div);
wrap_binary_assign!(RemAssign, rem_assign, wrapping_rem);

macro_rules! wrap_bit_binary {
    ($trait:ident, $method:ident, $op:tt) => {
        impl<T: Copy + $trait<Output = T>> $trait<Wrap<T>> for Wrap<T> {
            type Output = Wrap<T>;
            fn $method(self, rhs: Wrap<T>) -> Self::Output {
                Wrap(Wrapping(self.0 .0 $op rhs.0 .0))
            }
        }
        impl<T: Copy + $trait<Output = T>> $trait<T> for Wrap<T> {
            type Output = Wrap<T>;
            fn $method(self, rhs: T) -> Self::Output {
                Wrap(Wrapping(self.0 .0 $op rhs))
            }
        }
    };
}

wrap_bit_binary!(BitAnd, bitand, &);
wrap_bit_binary!(BitOr, bitor, |);
wrap_bit_binary!(BitXor, bitxor, ^);

macro_rules! wrap_bit_binary_assign {
    ($bound:ident, $trait:ident, $method:ident, $op:tt) => {
        impl<T: Copy + $bound<Output = T>> $trait<Wrap<T>> for Wrap<T> {
            fn $method(&mut self, rhs: Wrap<T>) {
                self.0 = Wrapping(self.0 .0 $op rhs.0 .0);
            }
        }
        impl<T: Copy + $bound<Output = T>> $trait<T> for Wrap<T> {
            fn $method(&mut self, rhs: T) {
                self.0 = Wrapping(self.0 .0 $op rhs);
            }
        }
    };
}

wrap_bit_binary_assign!(BitAnd, BitAndAssign, bitand_assign, &);
wrap_bit_binary_assign!(BitOr, BitOrAssign, bitor_assign, |);
wrap_bit_binary_assign!(BitXor, BitXorAssign, bitxor_assign, ^);

macro_rules! wrap_shift {
    ($trait:ident, $method:ident, $assign_trait:ident, $assign_method:ident, $op:tt) => {
        impl<T, U> $trait<U> for Wrap<T>
        where
            T: $trait<U, Output = T>,
        {
            type Output = Wrap<T>;
            fn $method(self, rhs: U) -> Self::Output {
                Wrap(Wrapping(self.0 .0 $op rhs))
            }
        }
        impl<T, U> $assign_trait<U> for Wrap<T>
        where
            T: Copy + $trait<U, Output = T>,
        {
            fn $assign_method(&mut self, rhs: U) {
                self.0 = Wrapping(self.0 .0 $op rhs);
            }
        }
    };
}

wrap_shift!(Shl, shl, ShlAssign, shl_assign, <<);
wrap_shift!(Shr, shr, ShrAssign, shr_assign, >>);

/// A wrapper around [`std::num::Wrapping<T>`] with ergonomic operator overloading.
///
/// `Wrap<T>` supports arithmetic, bitwise, and shift operations with both
/// other `Wrap<T>` values and plain `T` values, while preserving wrapping semantics.
///
/// The inner [`std::num::Wrapping<T>`] can be accessed via the public `.0` field,
/// or converted back using [`From<Wrap<T>> for Wrapping<T>`] or [`Wrap::into_inner`].
///
/// [`std::num::Wrapping<T>`]: std::num::Wrapping
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Wrap<T>(pub Wrapping<T>);

impl<T> Wrap<T> {
    /// Creates a new `Wrap<T>` from a value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use wrapn::Wrap;
    /// let w = Wrap::new(42u32);
    /// ```
    pub fn new(value: T) -> Self {
        Wrap(Wrapping(value))
    }

    /// Consumes the `Wrap<T>` and returns the inner value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use wrapn::Wrap;
    /// let w = Wrap::new(42u32);
    /// assert_eq!(w.into_inner(), 42u32);
    /// ```
    pub fn into_inner(self) -> T {
        self.0 .0
    }
}

impl<T> From<T> for Wrap<T> {
    fn from(value: T) -> Self {
        Wrap::new(value)
    }
}

impl<T> From<Wrapping<T>> for Wrap<T> {
    fn from(w: Wrapping<T>) -> Self {
        Wrap(w)
    }
}

impl<T> From<Wrap<T>> for Wrapping<T> {
    fn from(w: Wrap<T>) -> Self {
        w.0
    }
}

impl<T> Display for Wrap<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T: Copy + Num> Neg for Wrap<T> {
    type Output = Wrap<T>;
    fn neg(self) -> Self::Output {
        Wrap(Wrapping(self.0 .0.wrapping_neg()))
    }
}

impl<T: Copy + Not<Output = T>> Not for Wrap<T> {
    type Output = Wrap<T>;
    fn not(self) -> Self::Output {
        Wrap(Wrapping(!self.0 .0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wrap;

    #[track_caller]
    fn assert_wrap_eq<T: Copy + Num + std::fmt::Debug + PartialEq>(
        a: Wrap<T>,
        b: Wrap<T>,
    ) {
        assert_eq!(a.into_inner(), b.into_inner());
    }

    #[test]
    fn add_wrap_wrap() {
        assert_wrap_eq(Wrap::new(5u32) + Wrap::new(3u32), Wrap::new(8u32));
    }

    #[test]
    fn add_wrap_t() {
        assert_wrap_eq(Wrap::new(5u32) + 3u32, Wrap::new(8u32));
        assert_wrap_eq(Wrap::new(u32::MAX) + 1u32, Wrap::new(0u32));
    }

    #[test]
    fn add_assign_wrap() {
        let mut x = Wrap::new(5u32);
        x += Wrap::new(3u32);
        assert_wrap_eq(x, Wrap::new(8u32));
    }

    #[test]
    fn add_assign_t() {
        let mut x = Wrap::new(5u32);
        x += 3u32;
        assert_wrap_eq(x, Wrap::new(8u32));
    }

    #[test]
    fn sub_wrap_wrap() {
        assert_wrap_eq(Wrap::new(10u32) - Wrap::new(3u32), Wrap::new(7u32));
    }

    #[test]
    fn sub_wrap_t() {
        assert_wrap_eq(Wrap::new(5u32) - 3u32, Wrap::new(2u32));
    }

    #[test]
    fn mul_wrap_wrap() {
        assert_wrap_eq(Wrap::new(6u32) * Wrap::new(7u32), Wrap::new(42u32));
    }

    #[test]
    fn mul_wrap_t() {
        assert_wrap_eq(Wrap::new(6u32) * 7u32, Wrap::new(42u32));
    }

    #[test]
    fn div_wrap_wrap() {
        assert_wrap_eq(Wrap::new(42u32) / Wrap::new(7u32), Wrap::new(6u32));
    }

    #[test]
    fn div_wrap_t() {
        assert_wrap_eq(Wrap::new(42u32) / 7u32, Wrap::new(6u32));
    }

    #[test]
    fn rem_wrap_wrap() {
        assert_wrap_eq(Wrap::new(10u32) % Wrap::new(3u32), Wrap::new(1u32));
    }

    #[test]
    fn rem_wrap_t() {
        assert_wrap_eq(Wrap::new(10u32) % 3u32, Wrap::new(1u32));
    }

    #[test]
    fn bitand_wrap_wrap() {
        assert_wrap_eq(
            Wrap::new(0b1100u32) & Wrap::new(0b1010u32),
            Wrap::new(0b1000u32),
        );
    }

    #[test]
    fn bitand_wrap_t() {
        assert_wrap_eq(Wrap::new(0b1100u32) & 0b1010u32, Wrap::new(0b1000u32));
    }

    #[test]
    fn bitor_wrap_wrap() {
        assert_wrap_eq(
            Wrap::new(0b1100u32) | Wrap::new(0b1010u32),
            Wrap::new(0b1110u32),
        );
    }

    #[test]
    fn bitor_wrap_t() {
        assert_wrap_eq(Wrap::new(0b1100u32) | 0b1010u32, Wrap::new(0b1110u32));
    }

    #[test]
    fn bitxor_wrap_wrap() {
        assert_wrap_eq(
            Wrap::new(0b1100u32) ^ Wrap::new(0b1010u32),
            Wrap::new(0b0110u32),
        );
    }

    #[test]
    fn bitxor_wrap_t() {
        assert_wrap_eq(Wrap::new(0b1100u32) ^ 0b1010u32, Wrap::new(0b0110u32));
    }

    #[test]
    fn shl_wrap_t() {
        assert_wrap_eq(Wrap::new(1u32) << 3u32, Wrap::new(8u32));
    }

    #[test]
    fn shr_wrap_t() {
        assert_wrap_eq(Wrap::new(8u32) >> 3u32, Wrap::new(1u32));
    }

    #[test]
    fn neg_wrap() {
        assert_wrap_eq(-Wrap::new(5i32), Wrap::new(-5i32));
    }

    #[test]
    fn not_wrap() {
        assert_wrap_eq(!Wrap::new(0b1111u32), Wrap::new(!0b1111u32));
    }

    #[test]
    fn into_inner() {
        assert_eq!(Wrap::new(42u32).into_inner(), 42u32);
    }

    #[test]
    fn from_conversions() {
        let w: Wrap<u32> = 42u32.into();
        assert_wrap_eq(w, Wrap::new(42u32));

        let w2: Wrap<u32> = Wrapping(42u32).into();
        assert_wrap_eq(w2, Wrap::new(42u32));

        let inner: Wrapping<u32> = w.into();
        assert_eq!(inner, Wrapping(42u32));
    }

    #[test]
    fn partial_eq() {
        assert!(Wrap::new(5u32) == Wrap::new(5u32));
        assert!(Wrap::new(5u32) != Wrap::new(3u32));
    }

    #[test]
    fn partial_ord() {
        assert!(Wrap::new(5u32) < Wrap::new(10u32));
        assert!(Wrap::new(10u32) > Wrap::new(5u32));
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", Wrap::new(42u32)), "42");
    }

    #[test]
    fn wrapping_behavior() {
        assert_wrap_eq(Wrap::new(u8::MAX) + 1u8, Wrap::new(0u8));
        assert_wrap_eq(-Wrap::new(i8::MIN), Wrap::new(i8::MIN));
    }

    #[test]
    fn shl_assign() {
        let mut x = Wrap::new(1u32);
        x <<= 3u32;
        assert_wrap_eq(x, Wrap::new(8u32));
    }

    #[test]
    fn shr_assign() {
        let mut x = Wrap::new(8u32);
        x >>= 3u32;
        assert_wrap_eq(x, Wrap::new(1u32));
    }

    #[test]
    fn wrap_macro() {
        let score = wrap!(10u32);
        assert_eq!(score, Wrap::new(10u32));
    }
}
