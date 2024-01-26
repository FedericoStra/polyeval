//! Evaluate polynomials.
//!
//! - <https://en.wikipedia.org/wiki/Polynomial_evaluation>
//! - <https://en.wikipedia.org/wiki/Horner%27s_method>
//! - <https://en.wikipedia.org/wiki/Estrin%27s_scheme>
//! - <https://hal.science/file/index/docid/846961/filename/fast_polynomial.pdf>
//!
//! # Usage
//!
//! # Examples

#![cfg_attr(docsrs, feature(doc_auto_cfg))]

/// Evaluate a polynomial with [Horner's method](https://en.wikipedia.org/wiki/Horner%27s_method).
///
/// The coefficients are listed from zeroth order to highest.
///
/// At least one coefficient must be present;
/// the coefficients may be enclosed in square brackets;
/// a trailing comma is allowed at the end of the coefficients.
///
/// # Usage
///
/// The macro can be invoked in any of the following ways:
/// - `horner!(x; a₀, a₁, ..., aₙ)`,
/// - `horner!(x; a₀, a₁, ..., aₙ,)`,
/// - `horner!(x; [a₀, a₁, ..., aₙ])`,
/// - `horner!(x; [a₀, a₁, ..., aₙ,])`.
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
    ($x:expr; [$($coeffs:expr),+ $(,)?]) => { horner!($x; $($coeffs),*) };
    ($x:expr; $a:expr $(,)?) => { $a };
    ($x:expr; $a:expr, $($rest:expr),+ $(,)?) => { $a + $x * horner!($x; $($rest),+) };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_horner() {
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
}
