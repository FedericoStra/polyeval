//! Evaluate polynomials.
//!
//! [![crates.io](https://img.shields.io/crates/v/polyeval?logo=rust)](https://crates.io/crates/polyeval)
//! [![docs.rs](https://img.shields.io/docsrs/polyeval?logo=docsdotrs)](https://docs.rs/polyeval)
//! [![GitHub](https://img.shields.io/static/v1?label=github&message=FedericoStra/polyeval&color=brightgreen&logo=github)](https://github.com/FedericoStra/polyeval)
//! [![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/FedericoStra/polyeval/rust.yml?logo=githubactions&logoColor=white)](https://github.com/FedericoStra/polyeval/actions/workflows/rust.yml)
//! [![Dependencies status](https://deps.rs/repo/github/FedericoStra/polyeval/status.svg)](https://deps.rs/repo/github/FedericoStra/polyeval)
//! [![MIT license](https://img.shields.io/crates/l/polyeval)](https://choosealicense.com/licenses/mit/)
//!
//! - <https://en.wikipedia.org/wiki/Polynomial_evaluation>
//! - <https://en.wikipedia.org/wiki/Horner%27s_method>
//! - <https://en.wikipedia.org/wiki/Estrin%27s_scheme>
//! - <https://hal.science/file/index/docid/846961/filename/fast_polynomial.pdf>
//!
//! # Usage
//!
//! # Examples

#![cfg_attr(not(any(feature = "std", test)), no_std)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

use core::ops::{Add, Mul};
use num_traits::{MulAdd, Zero};

#[inline]
#[doc(hidden)]
pub fn mul_add<T: MulAdd<Output = T>>(x: T, a: T, b: T) -> T {
    x.mul_add(a, b)
}

/// Evaluate a polynomial with [Horner's method](https://en.wikipedia.org/wiki/Horner%27s_method).
///
/// The coefficients are listed from zeroth order to highest.
///
/// # Examples
///
/// ```
/// use polyeval::horner;
///
/// let x = 7;
///
/// assert_eq!(horner(x, &[]), 0);
/// assert_eq!(horner(x, &[0]), 0);
///
/// assert_eq!(
///     horner(x, &[2, 3, 4]),
///     2 + x * (3 + x * 4)
/// );
/// ```
pub fn horner<T>(x: T, coeffs: &[T]) -> T
where
    T: Zero,
    T: for<'a> Add<&'a T, Output = T>,
    T: for<'a> Mul<&'a T, Output = T>,
{
    coeffs.iter().rfold(T::zero(), |acc: T, c: &T| acc * &x + c)
}

/// Evaluate a polynomial with [Horner's method](https://en.wikipedia.org/wiki/Horner%27s_method).
///
/// Same as [`horner`](fn@horner), but specialized for [arrays](primitive@array).
/// At the moment this doesn't seem to provide better codegen, but it may change in the future.
///
/// Currently, the only benefit over [`horner`](fn@horner) is that it provides
/// type-checked verification of the order of the polynomial.
///
/// # Examples
///
/// ```
/// use polyeval::horner_array;
///
/// let x = 7;
///
/// assert_eq!(
///     // if we so desire, we can type-check the order of the polynomial:
///     //                v-- this is one plus the order of the polynomial
///     horner_array::<_, 3>(x, &[2, 3, 4]),
///     2 + x * (3 + x * 4)
/// );
/// ```
pub fn horner_array<T, const N: usize>(x: T, coeffs: &[T; N]) -> T
where
    T: Zero,
    T: for<'a> Add<&'a T, Output = T>,
    T: for<'a> Mul<&'a T, Output = T>,
{
    coeffs.iter().rfold(T::zero(), |acc: T, c: &T| acc * &x + c)
}

/// Evaluate a polynomial with [Horner's method](https://en.wikipedia.org/wiki/Horner%27s_method).
///
/// The coefficients are listed from zeroth order to highest.
///
/// At least one coefficient must be present;
/// the coefficients may be enclosed in square brackets;
/// a trailing comma is allowed at the end of the coefficients.
///
/// If the first token is the keyword `let`, then the `x` argument is evaluated only once.
///
/// # Usage
///
/// The macro can be invoked in any of the following ways:
/// - `horner!(x; a₀, a₁, ..., aₙ)`,
/// - `horner!(x; a₀, a₁, ..., aₙ,)`,
/// - `horner!(x; [a₀, a₁, ..., aₙ])`,
/// - `horner!(x; [a₀, a₁, ..., aₙ,])`,
/// - `horner!(let x; a₀, a₁, ..., aₙ)`,
/// - `horner!(let x; a₀, a₁, ..., aₙ,)`,
/// - `horner!(let x; [a₀, a₁, ..., aₙ])`,
/// - `horner!(let x; [a₀, a₁, ..., aₙ,])`.
///
/// In every case it computes the polynomial `aₙxⁿ + ... + a₁x + a₀` using Horner's method, i.e.
/// `a₀ + x*(a₁ + x*( ... + x*aₙ...))`.
///
/// # Examples
///
/// ```
/// use polyeval::horner;
///
/// let x = 7;
///
/// assert_eq!(horner!(x; [0]), 0);
/// assert_eq!(horner!(x; [0,]), 0);
/// assert_eq!(horner!(x; 0), 0);
/// assert_eq!(horner!(x; 0,), 0);
///
/// assert_eq!(
///     horner!(x; 2, 3, 4),
///     2 + x * (3 + x * 4)
/// );
///
/// assert_eq!(
///     horner!(x; [2, 3, 4]),
///     2 + x * (3 + x * 4)
/// );
/// ```
#[macro_export]
macro_rules! horner {
    // ($x:expr; ) => (0); // `0` should be of the same type as `x`...
    ($x:expr; [$($coeffs:expr),+ $(,)?]) => { $crate::horner!($x; $($coeffs),*) };
    ($x:expr; $a:expr $(,)?) => { $a };
    ($x:expr; $a:expr, $($rest:expr),+ $(,)?) => { $a + $x * $crate::horner!($x; $($rest),+) };
    (let $x:expr; $($t:tt)*) => {{
        let x = $x;
        $crate::horner!(x; $($t)*)
    }};
}

/// Evaluate a polynomial with [Horner's method](https://en.wikipedia.org/wiki/Horner%27s_method).
///
/// Same as [`horner!`], but uses "fused multiply-add" instructions.
///
/// The coefficients and the point of evaluation must be of a type which implements [`num_traits::MulAdd`].
#[macro_export]
macro_rules! horner_fma {
    // ($x:expr; ) => (0); // `0` should be of the same type as `x`... Maybe return `x-x`?
    ($x:expr; [$($coeffs:expr),+ $(,)?]) => { $crate::horner_fma!($x; $($coeffs),*) };
    ($x:expr; $a:expr $(,)?) => { $a };
    ($x:expr; $a:expr, $($rest:expr),+ $(,)?) => { $crate::mul_add($crate::horner_fma!($x; $($rest),+), $x, $a) };
    (let $x:expr; $($t:tt)*) => {{
        let x = $x;
        $crate::horner_fma!(x; $($t)*)
    }};
}

/// Evaluate a polynomial with [Estrin's scheme](https://en.wikipedia.org/wiki/Estrin%27s_scheme).
///
/// The coefficients are listed from zeroth order to highest.
///
/// At least one coefficient must be present;
/// the coefficients may be enclosed in square brackets;
/// a trailing comma is allowed at the end of the coefficients.
///
/// If the first token is the keyword `let`, then the `x` argument is evaluated only once.
///
/// # Usage
///
/// The macro can be invoked in any of the following ways:
/// - `estrin!(x; a₀, a₁, ..., aₙ)`,
/// - `estrin!(x; a₀, a₁, ..., aₙ,)`,
/// - `estrin!(x; [a₀, a₁, ..., aₙ])`,
/// - `estrin!(x; [a₀, a₁, ..., aₙ,])`,
/// - `estrin!(let x; a₀, a₁, ..., aₙ)`,
/// - `estrin!(let x; a₀, a₁, ..., aₙ,)`,
/// - `estrin!(let x; [a₀, a₁, ..., aₙ])`,
/// - `estrin!(let x; [a₀, a₁, ..., aₙ,])`.
///
/// In every case it computes the polynomial `aₙxⁿ + ... + a₁x + a₀` using Estrin's scheme.
///
/// # Examples
///
/// ```
/// use polyeval::estrin;
///
/// let x = 7;
///
/// assert_eq!(estrin!(x; [0]), 0);
/// assert_eq!(estrin!(x; [0,]), 0);
/// assert_eq!(estrin!(x; 0), 0);
/// assert_eq!(estrin!(x; 0,), 0);
///
/// assert_eq!(
///     estrin!(x; 2, 3, 4),
///     2 + x * (3 + x * 4)
/// );
///
/// assert_eq!(
///     estrin!(x; [2, 3, 4]),
///     2 + x * (3 + x * 4)
/// );
/// ```
#[macro_export]
macro_rules! estrin {
    // ($x:expr; ) => (0); // `0` should be of the same type as `x`... Maybe return `x-x`?
    ($x:expr; [$($coeffs:expr),* $(,)?]) => { $crate::estrin!($x; $($coeffs),*) };
    ($x:expr; $a:expr $(,)?) => { $a };
    ($x:expr; $a0:expr, $a1:expr $(,)?) => { $a0 + $x * $a1 };
    ($x:expr; $($coeffs:expr),+ $(,)?) => { $crate::estrin!($x; $($coeffs),+; ) };
    (let $x:expr; $($t:tt)*) => {{
        let x = $x;
        $crate::estrin!(x; $($t)*)
    }};

    // one coefficient left
    ($x:expr; $a0:expr; $($out:expr),+) => {{
        let x = $x*$x;
        $crate::estrin!(x; $($out),+, $a0)
    }};
    // two coefficients left
    ($x:expr; $a0:expr, $a1:expr; $($out:expr),+) => {{
        let x = $x*$x;
        $crate::estrin!(x; $($out),+, $a0 + $x * $a1)
    }};
    // more coefficients left
    ($x:expr; $a0:expr, $a1:expr, $($rest:expr),+; ) => {
        $crate::estrin!($x; $($rest),+; $a0 + $x * $a1)
    };
    ($x:expr; $a0:expr, $a1:expr, $($rest:expr),+; $($out:expr),+) => {
        $crate::estrin!($x; $($rest),+; $($out),+, $a0 + $x * $a1)
    };
}

/// Evaluate a polynomial with [Estrin's scheme](https://en.wikipedia.org/wiki/Estrin%27s_scheme).
///
/// Same as [`estrin!`], but uses "fused multiply-add" instructions.
///
/// The coefficients and the point of evaluation must be of a type which implements [`num_traits::MulAdd`].
#[macro_export]
macro_rules! estrin_fma {
    // ($x:expr; ) => (0); // `0` should be of the same type as `x`... Maybe return `x-x`?
    ($x:expr; [$($coeffs:expr),* $(,)?]) => { $crate::estrin_fma!($x; $($coeffs),*) };
    ($x:expr; $a:expr $(,)?) => { $a };
    ($x:expr; $a0:expr, $a1:expr $(,)?) => { $crate::mul_add($x, $a1, $a0) };
    ($x:expr; $($coeffs:expr),+ $(,)?) => { $crate::estrin_fma!($x; $($coeffs),+; ) };
    (let $x:expr; $($t:tt)*) => {{
        let x = $x;
        $crate::estrin_fma!(x; $($t)*)
    }};

    // one coefficient left
    ($x:expr; $a0:expr; $($out:expr),+) => {{
        let x = $x*$x;
        $crate::estrin_fma!(x; $($out),+, $a0)
    }};
    // two coefficients left
    ($x:expr; $a0:expr, $a1:expr; $($out:expr),+) => {{
        let x = $x*$x;
        $crate::estrin_fma!(x; $($out),+, $crate::mul_add($x, $a1, $a0))
    }};
    // more coefficients left
    ($x:expr; $a0:expr, $a1:expr, $($rest:expr),+; ) => {
        $crate::estrin_fma!($x; $($rest),+; $crate::mul_add($x, $a1, $a0))
    };
    ($x:expr; $a0:expr, $a1:expr, $($rest:expr),+; $($out:expr),+) => {
        $crate::estrin_fma!($x; $($rest),+; $($out),+, $crate::mul_add($x, $a1, $a0))
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_horner() {
        use super::horner;
        for x in 0..32 {
            assert_eq!(horner(x, &[]), 0);
            assert_eq!(horner(x, &[1]), 1);
            assert_eq!(horner(x, &[1, 2]), 1 + 2 * x);
            assert_eq!(
                horner(x, &[1, 2, 3, 4, 5]),
                1 + x * (2 + x * (3 + x * (4 + x * 5)))
            );
        }
        for x in 0..32 {
            let x = x as f32;
            assert_eq!(horner(x, &[1.]), 1.);
            assert_eq!(horner(x, &[1., 2.]), 1. + 2. * x);
            assert_eq!(
                horner(x, &[1., 2., 3., 4., 5.]),
                1. + x * (2. + x * (3. + x * (4. + x * 5.)))
            );
        }
    }

    #[test]
    fn test_horner_array() {
        use super::horner_array;
        for x in 0..32 {
            assert_eq!(horner_array(x, &[]), 0);
            assert_eq!(horner_array(x, &[1]), 1);
            assert_eq!(horner_array(x, &[1, 2]), 1 + 2 * x);
            assert_eq!(
                horner_array(x, &[1, 2, 3, 4, 5]),
                1 + x * (2 + x * (3 + x * (4 + x * 5)))
            );
        }
        for x in 0..32 {
            let x = x as f32;
            assert_eq!(horner_array(x, &[1.]), 1.);
            assert_eq!(horner_array(x, &[1., 2.]), 1. + 2. * x);
            assert_eq!(
                horner_array(x, &[1., 2., 3., 4., 5.]),
                1. + x * (2. + x * (3. + x * (4. + x * 5.)))
            );
            assert_eq!(
                horner_array::<_, 5>(x, &[1., 2., 3., 4., 5.]),
                1. + x * (2. + x * (3. + x * (4. + x * 5.)))
            );
            assert_eq!(
                horner_array::<f32, 5>(x, &[1., 2., 3., 4., 5.]),
                1. + x * (2. + x * (3. + x * (4. + x * 5.)))
            );
        }
    }

    #[test]
    fn test_macro_horner() {
        for x in 0..32 {
            assert_eq!(horner!(x; 1), 1);
            assert_eq!(horner!(x; 1,), 1);
            assert_eq!(horner!(x; 1, 2), 1 + 2 * x);
            assert_eq!(horner!(x; 1, 2,), 1 + 2 * x);
            assert_eq!(
                horner!(x; 1, 2, 3, 4, 5),
                1 + x * (2 + x * (3 + x * (4 + x * 5)))
            );
        }
        for x in 0..32 {
            let x = x as f32;
            assert_eq!(horner!(x; 1.), 1.);
            assert_eq!(horner!(x; 1.,), 1.);
            assert_eq!(horner!(x; 1., 2.), 1. + 2. * x);
            assert_eq!(horner!(x; 1., 2.,), 1. + 2. * x);
            assert_eq!(
                horner!(x; 1., 2., 3., 4., 5.),
                1. + x * (2. + x * (3. + x * (4. + x * 5.)))
            );
        }
    }

    #[cfg(any(feature = "std", feature = "lib"))]
    #[test]
    fn test_macro_horner_fma() {
        for x in 0..32 {
            assert_eq!(horner_fma!(x; 1), 1);
            assert_eq!(horner_fma!(x; 1,), 1);
            assert_eq!(horner_fma!(x; 1, 2), 1 + 2 * x);
            assert_eq!(horner_fma!(x; 1, 2,), 1 + 2 * x);
            assert_eq!(
                horner_fma!(x; 1, 2, 3, 4, 5),
                1 + x * (2 + x * (3 + x * (4 + x * 5)))
            );
        }
        for x in 0..32 {
            let x = x as f32;
            assert_eq!(horner_fma!(x; 1.), 1.);
            assert_eq!(horner_fma!(x; 1.,), 1.);
            assert_eq!(horner_fma!(x; 1., 2.), 1. + 2. * x);
            assert_eq!(horner_fma!(x; 1., 2.,), 1. + 2. * x);
            assert_eq!(
                horner_fma!(x; 1., 2., 3., 4., 5.),
                1. + x * (2. + x * (3. + x * (4. + x * 5.)))
            );
        }
    }

    #[test]
    fn test_macro_estrin() {
        for x in 0..32 {
            assert_eq!(estrin!(x; 1), 1);
            assert_eq!(estrin!(x; 1,), 1);
            assert_eq!(estrin!(x; 1, 2), 1 + 2 * x);
            assert_eq!(estrin!(x; 1, 2,), 1 + 2 * x);
            assert_eq!(
                estrin!(x; 1, 2, 3, 4, 5),
                1 + x * (2 + x * (3 + x * (4 + x * 5)))
            );
        }
        for x in 0..32 {
            let x = x as f32;
            assert_eq!(estrin!(x; 1.), 1.);
            assert_eq!(estrin!(x; 1.,), 1.);
            assert_eq!(estrin!(x; 1., 2.), 1. + 2. * x);
            assert_eq!(estrin!(x; 1., 2.,), 1. + 2. * x);
            assert_eq!(
                estrin!(x; 1., 2., 3., 4., 5.),
                1. + x * (2. + x * (3. + x * (4. + x * 5.)))
            );
        }
    }

    #[cfg(any(feature = "std", feature = "lib"))]
    #[test]
    fn test_macro_estrin_fma() {
        for x in 0..32 {
            assert_eq!(estrin_fma!(x; 1), 1);
            assert_eq!(estrin_fma!(x; 1,), 1);
            assert_eq!(estrin_fma!(x; 1, 2), 1 + 2 * x);
            assert_eq!(estrin_fma!(x; 1, 2,), 1 + 2 * x);
            assert_eq!(
                estrin_fma!(x; 1, 2, 3, 4, 5),
                1 + x * (2 + x * (3 + x * (4 + x * 5)))
            );
        }
        for x in 0..32 {
            let x = x as f32;
            assert_eq!(estrin_fma!(x; 1.), 1.);
            assert_eq!(estrin_fma!(x; 1.,), 1.);
            assert_eq!(estrin_fma!(x; 1., 2.), 1. + 2. * x);
            assert_eq!(estrin_fma!(x; 1., 2.,), 1. + 2. * x);
            assert_eq!(
                estrin_fma!(x; 1., 2., 3., 4., 5.),
                1. + x * (2. + x * (3. + x * (4. + x * 5.)))
            );
        }
    }

    fn make_callable_only_once() -> impl FnMut() -> i32 {
        let mut called = false;
        move || {
            if !called {
                called = true;
                1
            } else {
                panic!("function called more than once")
            }
        }
    }

    #[test]
    #[should_panic(expected = "function called more than once")]
    fn test_macro_horner_cannot_call_more_than_once() {
        let mut f = make_callable_only_once();
        assert_eq!(horner!(f(); 1, 2, 3), 6);
    }

    #[test]
    fn test_macro_horner_can_call_more_than_once() {
        let mut f = make_callable_only_once();
        assert_eq!(horner!(let f(); 1, 2, 3), 6);
        let mut f = make_callable_only_once();
        assert_eq!(horner!(let f(); [1, 2, 3]), 6);
    }

    #[cfg(any(feature = "std", feature = "lib"))]
    #[test]
    #[should_panic(expected = "function called more than once")]
    fn test_macro_horner_fma_cannot_call_more_than_once() {
        let mut f = make_callable_only_once();
        assert_eq!(horner_fma!(f(); 1, 2, 3), 6);
    }

    #[cfg(any(feature = "std", feature = "lib"))]
    #[test]
    fn test_macro_horner_fma_can_call_more_than_once() {
        let mut f = make_callable_only_once();
        assert_eq!(horner_fma!(let f(); 1, 2, 3), 6);
        let mut f = make_callable_only_once();
        assert_eq!(horner_fma!(let f(); [1, 2, 3]), 6);
    }

    #[test]
    #[should_panic(expected = "function called more than once")]
    fn test_macro_estrin_cannot_call_more_than_once() {
        let mut f = make_callable_only_once();
        assert_eq!(estrin!(f(); 1, 2, 3), 6);
    }

    #[test]
    fn test_macro_estrin_can_call_more_than_once() {
        let mut f = make_callable_only_once();
        assert_eq!(estrin!(let f(); 1, 2, 3), 6);
        let mut f = make_callable_only_once();
        assert_eq!(estrin!(let f(); [1, 2, 3]), 6);
    }

    #[cfg(any(feature = "std", feature = "lib"))]
    #[test]
    #[should_panic(expected = "function called more than once")]
    fn test_macro_estrin_fma_cannot_call_more_than_once() {
        let mut f = make_callable_only_once();
        assert_eq!(estrin_fma!(f(); 1, 2, 3), 6);
    }

    #[cfg(any(feature = "std", feature = "lib"))]
    #[test]
    fn test_macro_estrin_fma_can_call_more_than_once() {
        let mut f = make_callable_only_once();
        assert_eq!(estrin_fma!(let f(); 1, 2, 3), 6);
        let mut f = make_callable_only_once();
        assert_eq!(estrin_fma!(let f(); [1, 2, 3]), 6);
    }
}
