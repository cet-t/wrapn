/// Creates a new `Wrap<T>` from a value.
///
/// # Example
///
/// ```rust
/// use wrapn::wrap;
/// let score = wrap!(10u32);
/// ```
#[macro_export]
macro_rules! wrap {
    ($val:expr) => {
        $crate::Wrap::new($val)
    };

    ($val:expr; $n:expr) => {
        [$crate::Wrap::new($val); $n]
    };

    ($($val:expr),+ $(,)?) => {
        [$($crate::Wrap::new($val)),+]
    };
}
