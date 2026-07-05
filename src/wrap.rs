use std::{
    fmt::{self, Display, Formatter},
    num::Wrapping,
    ops::*,
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
    fn rotate_left(self, rhs: u32) -> Self;
    fn rotate_right(self, rhs: u32) -> Self;
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
            fn rotate_left(self, rhs: u32) -> Self { self.rotate_left(rhs) }
            fn rotate_right(self, rhs: u32) -> Self { self.rotate_right(rhs) }
        })*
    };
}

num_impl! {
    u8, u16, u32, u64, u128, usize,
    i8, i16, i32, i64, i128, isize,
}

macro_rules! wrap_binary {
    ($name:ident) => {
        ::paste::paste! {
            impl<T: Copy + Num> [<$name>]<Wrap<T>> for Wrap<T> {
                type Output = Wrap<T>;
                fn [<$name:lower>](self, rhs: Wrap<T>) -> Self::Output {
                    Wrap(Wrapping(self.0 .0.[< wrapping_ $name:lower >](rhs.0 .0)))
                }
            }
            impl<T: Copy + Num> [<$name>]<T> for Wrap<T> {
                type Output = Wrap<T>;
                fn [<$name:lower>](self, rhs: T) -> Self::Output {
                    Wrap(Wrapping(self.0 .0.[< wrapping_ $name:lower >](rhs)))
                }
            }
        }
    };
}

macro_rules! wrap_binary_assign {
    ($name:ident) => {
        ::paste::paste! {
            impl<T: Copy + Num> [<$name Assign>]<Wrap<T>> for Wrap<T> {
                fn [<$name:lower _assign>](&mut self, rhs: Wrap<T>) {
                    self.0 = Wrapping(self.0 .0.[< wrapping_ $name:lower >](rhs.0 .0));
                }
            }
            impl<T: Copy + Num> [<$name Assign>]<T> for Wrap<T> {
                fn [<$name:lower _assign>](&mut self, rhs: T) {
                    self.0 = Wrapping(self.0 .0.[< wrapping_ $name:lower >](rhs));
                }
            }
        }
    };
}

wrap_binary!(Add);
wrap_binary!(Sub);
wrap_binary!(Mul);
wrap_binary!(Div);
wrap_binary!(Rem);

wrap_binary_assign!(Add);
wrap_binary_assign!(Sub);
wrap_binary_assign!(Mul);
wrap_binary_assign!(Div);
wrap_binary_assign!(Rem);

macro_rules! wrap_bit_binary {
    ($name:ident, $op:tt) => {
        ::paste::paste! {
            impl<T: Copy + $name<Output = T>> $name<Wrap<T>> for Wrap<T> {
                type Output = Wrap<T>;
                fn [<$name:lower>](self, rhs: Wrap<T>) -> Self::Output {
                    Wrap(Wrapping(self.0 .0 $op rhs.0 .0))
                }
            }
            impl<T: Copy + $name<Output = T>> $name<T> for Wrap<T> {
                type Output = Wrap<T>;
                fn [<$name:lower>](self, rhs: T) -> Self::Output {
                    Wrap(Wrapping(self.0 .0 $op rhs))
                }
            }
        }
    };
}

macro_rules! wrap_bit_binary_assign {
    ($name:ident, $op:tt) => {
        ::paste::paste! {
            impl<T: Copy + $name<Output = T>> [<$name Assign>]<Wrap<T>> for Wrap<T> {
                fn [<$name:lower _assign>](&mut self, rhs: Wrap<T>) {
                    self.0 = Wrapping(self.0 .0 $op rhs.0 .0);
                }
            }
            impl<T: Copy + $name<Output = T>> [<$name Assign>]<T> for Wrap<T> {
                fn [<$name:lower _assign>](&mut self, rhs: T) {
                    self.0 = Wrapping(self.0 .0 $op rhs);
                }
            }
        }
    };
}

wrap_bit_binary!(BitAnd, &);
wrap_bit_binary!(BitOr, |);
wrap_bit_binary!(BitXor, ^);

wrap_bit_binary_assign!(BitAnd, &);
wrap_bit_binary_assign!(BitOr, |);
wrap_bit_binary_assign!(BitXor, ^);

macro_rules! wrap_shift {
    ($name:ident, $op:tt) => {
        ::paste::paste! {
            impl<T, U> $name<U> for Wrap<T>
            where
                T: $name<U, Output = T>,
            {
                type Output = Wrap<T>;
                fn [<$name:lower>](self, rhs: U) -> Self::Output {
                    Wrap(Wrapping(self.0 .0 $op rhs))
                }
            }
        }
    };
}

macro_rules! wrap_shift_assign {
    ($name:ident, $op:tt) => {
        ::paste::paste! {
            impl<T, U> [<$name Assign>]<U> for Wrap<T>
            where
                T: Copy + $name<U, Output = T>,
            {
                fn [<$name:lower _assign>](&mut self, rhs: U) {
                    self.0 = Wrapping(self.0 .0 $op rhs);
                }
            }
        }
    };
}

wrap_shift!(Shl, <<);
wrap_shift!(Shr, >>);
wrap_shift_assign!(Shl, <<);
wrap_shift_assign!(Shr, >>);

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
        self.0.0
    }

    /// Returns a reference to the inner value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use wrapn::Wrap;
    /// let w = Wrap::new(42u32);
    /// assert_eq!(w.raw(), &42u32);
    /// ```
    pub fn raw(&self) -> &T {
        &self.0.0
    }
}

impl<T: Copy + Num> Wrap<T> {
    /// Rotates the inner value left by `rhs` bits, keeping wrapping semantics.
    ///
    /// # Example
    ///
    /// ```rust
    /// use wrapn::Wrap;
    /// assert_eq!(Wrap::new(0x01u32).rotate_left(3), Wrap::new(0x08u32));
    /// ```
    pub fn rotate_left(self, rhs: u32) -> Self {
        Wrap(Wrapping(self.0.0.rotate_left(rhs)))
    }

    /// Rotates the inner value right by `rhs` bits, keeping wrapping semantics.
    ///
    /// # Example
    ///
    /// ```rust
    /// use wrapn::Wrap;
    /// assert_eq!(Wrap::new(0x08u32).rotate_right(3), Wrap::new(0x01u32));
    /// ```
    pub fn rotate_right(self, rhs: u32) -> Self {
        Wrap(Wrapping(self.0.0.rotate_right(rhs)))
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
        Wrap(Wrapping(self.0.0.wrapping_neg()))
    }
}

impl<T: Copy + Not<Output = T>> Not for Wrap<T> {
    type Output = Wrap<T>;
    fn not(self) -> Self::Output {
        Wrap(Wrapping(!self.0.0))
    }
}

impl<T: PartialEq> PartialEq<T> for Wrap<T> {
    fn eq(&self, other: &T) -> bool {
        self.0.0 == *other
    }
}

impl<T: PartialOrd> PartialOrd<T> for Wrap<T> {
    fn partial_cmp(&self, other: &T) -> Option<std::cmp::Ordering> {
        self.0.0.partial_cmp(other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wrap;

    #[track_caller]
    fn assert_wrap_eq<T: Copy + Num + std::fmt::Debug + PartialEq>(a: Wrap<T>, b: Wrap<T>) {
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
    fn raw() {
        assert_eq!(Wrap::new(42u32).raw(), &42u32);
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
    fn partial_eq_with_t() {
        assert!(Wrap::new(5u32) == 5u32);
        assert!(Wrap::new(5u32) != 3u32);
    }

    #[test]
    fn partial_ord() {
        assert!(Wrap::new(5u32) < Wrap::new(10u32));
        assert!(Wrap::new(10u32) > Wrap::new(5u32));
    }

    #[test]
    fn partial_ord_with_t() {
        assert!(Wrap::new(5u32) < 10u32);
        assert!(Wrap::new(5u32) <= 5u32);
        assert!(Wrap::new(10u32) > 5u32);
        assert!(Wrap::new(10u32) >= 10u32);
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
    fn rotate_left_wrap() {
        assert_wrap_eq(Wrap::new(0x01u32).rotate_left(3), Wrap::new(0x08u32));
    }

    #[test]
    fn rotate_right_wrap() {
        assert_wrap_eq(Wrap::new(0x08u32).rotate_right(3), Wrap::new(0x01u32));
    }

    #[test]
    fn wrap_macro() {
        let score = wrap!(10u32);
        assert_eq!(score, Wrap::new(10u32));
    }

    #[test]
    fn wrap_macro_array_list() {
        let arr: [Wrap<u32>; 3] = wrap![1u32, 2u32, 3u32];
        assert_eq!(arr[0].into_inner(), 1u32);
        assert_eq!(arr[1].into_inner(), 2u32);
        assert_eq!(arr[2].into_inner(), 3u32);
    }

    #[test]
    fn wrap_macro_array_repeat() {
        let arr: [Wrap<u32>; 3] = wrap![42u32; 3];
        assert_eq!(arr[0].into_inner(), 42u32);
        assert_eq!(arr[1].into_inner(), 42u32);
        assert_eq!(arr[2].into_inner(), 42u32);
    }
}
