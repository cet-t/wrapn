//! # wrapn
//!
//! `Wrap<T>` is an ergonomic wrapper around [`std::num::Wrapping<T>`].
//!
//! It lets you use wrapping arithmetic with plain `T` values without explicit casting,
//! while keeping true wrapping semantics.
//!
//! ## Example
//!
//! ```rust
//! use wrapn::{Wrap, wrap};
//!
//! let a = Wrap::new(5u32);
//! let b = 3u32;
//! let score = wrap!(10u32);
//!
//! // arithmetic with plain integers
//! assert_eq!(a + b, Wrap::new(8u32));
//! assert_eq!(Wrap::new(u32::MAX) + 1u32, Wrap::new(0u32));
//!
//! // bitwise ops
//! assert_eq!(Wrap::new(0b1100u32) & 0b1010u32, Wrap::new(0b1000u32));
//!
//! // shifts, negation, not
//! assert_eq!(Wrap::new(1u32) << 3u32, Wrap::new(8u32));
//! assert_eq!(-Wrap::new(5i32), Wrap::new(-5i32));
//! assert_eq!(!Wrap::new(0b1111u32), Wrap::new(!0b1111u32));
//! ```
//!
//! ## Supported operations
//!
//! - Arithmetic: `+`, `-`, `*`, `/`, `%`, plus assign variants
//! - Bitwise: `&`, `|`, `^`, plus assign variants
//! - Shifts: `<<`, `>>`, plus assign variants
//! - Unary: `-`, `!`
//! - Comparisons: `==`, `!=`, `<`, `<=`, `>`, `>=` against `T`
//! - Conversions: `From<T>`, `From<Wrapping<T>>`, `From<Wrap<T>> for Wrapping<T>`
//! - Traits: `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, `PartialOrd`, `Ord`, `Hash`, `Display`

mod macros;
mod wrap;
pub use wrap::*;
