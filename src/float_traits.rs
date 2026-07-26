// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Copyright (c) 2026 c1ph3rC4t

#[cfg(feature = "fraction")]
use fraction::BigFraction;
use std::ops::{Add, Div, DivAssign, Mul, Neg, Sub};

/// Trait for float types that support basic arithmetic and common math functions.
///
/// Implemented for `f32` and `f64` out of the box.
///
/// You can implement this for custom float types using the [`impl_float`] macro,
/// as long as the type supports the required operations and has inherent methods
/// for `sqrt`, `sin`, `cos`, `acos`, `ln`, and `exp`.
pub trait Float:
    Add<Output = Self>
    + Sub<Output = Self>
    + Neg<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + DivAssign
    + Clone
    + PartialOrd
    + From<f32>
    + Sized
{
    /// π/2
    fn frac_pi_2() -> Self;

    /// 0.0
    fn zero() -> Self;

    /// 1.0
    fn one() -> Self;

    /// 2.0
    fn two() -> Self;

    /// Square root.
    #[must_use]
    fn sqrt(self) -> Self;

    /// Sine (radians).
    #[must_use]
    fn sin(self) -> Self;

    /// Cosine (radians).
    #[must_use]
    fn cos(self) -> Self;

    /// Inverse cosine (radians).
    #[must_use]
    fn acos(self) -> Self;

    /// Natural logarithm.
    #[must_use]
    fn ln(self) -> Self;

    /// Exponential (e^self).
    #[must_use]
    fn exp(self) -> Self;

    /// Atan2 (radians).
    #[must_use]
    fn atan2(self, other: Self) -> Self;

    /// Absolute value.
    #[must_use]
    fn abs(self) -> Self;

    /// Formatter for quaternion coord.
    #[must_use]
    fn fmt_quaternion_coord(self) -> String;
}

/// Implements [`Float`] for some type more easily.
#[macro_export]
macro_rules! impl_float {
    ($t:ty, $frac_pi_2:expr, $formatter:expr) => {
        impl Float for $t {
            #[inline]
            fn frac_pi_2() -> Self {
                $frac_pi_2
            }
            #[inline]
            fn zero() -> Self {
                0.0
            }
            #[inline]
            fn one() -> Self {
                1.0
            }
            #[inline]
            fn two() -> Self {
                2.0
            }
            fn sqrt(self) -> Self {
                self.sqrt()
            }
            fn sin(self) -> Self {
                self.sin()
            }
            fn cos(self) -> Self {
                self.cos()
            }
            fn acos(self) -> Self {
                self.acos()
            }
            fn ln(self) -> Self {
                self.ln()
            }
            fn exp(self) -> Self {
                self.exp()
            }
            fn atan2(self, other: Self) -> Self {
                self.atan2(other)
            }
            fn abs(self) -> Self {
                self.abs()
            }
            fn fmt_quaternion_coord(self) -> String {
                ($formatter)(self)
            }
        }
    };
}

impl_float!(f32, std::f32::consts::FRAC_PI_2, |v| format!("{v}"));
impl_float!(f64, std::f64::consts::FRAC_PI_2, |v| format!("{v}"));
#[cfg(feature = "f128")]
impl_float!(f128, std::f128::consts::FRAC_PI_2, |v| format!(
    "{}",
    v as f64
));

#[cfg(feature = "fraction")]
fn f64_from_big_fraction(frac: &BigFraction) -> f64 {
    let s = frac.to_string();
    match s.split_once('/') {
        Some((n, d)) => {
            n.parse::<f64>().unwrap_or_else(|_| unreachable!())
                / d.parse::<f64>().unwrap_or_else(|_| unreachable!())
        }
        None => s.parse::<f64>().unwrap_or_else(|_| unreachable!()),
    }
}

#[cfg(feature = "fraction")]
macro_rules! impl_trig_fn {
    ($name:ident $(, $arg:ident)?) => {
        fn $name(self $(, $arg: Self)?) -> Self {
            Self::from(f64_from_big_fraction(&self).$name($(f64_from_big_fraction(&$arg))?))
        }
    };
}

#[cfg(feature = "fraction")]
impl Float for BigFraction {
    fn frac_pi_2() -> Self {
        Self::from(std::f64::consts::FRAC_PI_2)
    }
    fn zero() -> Self {
        Self::from(0)
    }
    fn one() -> Self {
        Self::from(1)
    }
    fn two() -> Self {
        Self::from(2)
    }
    fn sqrt(self) -> Self {
        Self::sqrt(&self, 128)
    }
    impl_trig_fn!(sin);
    impl_trig_fn!(cos);
    impl_trig_fn!(acos);
    impl_trig_fn!(ln);
    impl_trig_fn!(exp);
    impl_trig_fn!(atan2, other);
    fn abs(self) -> Self {
        if self < Self::zero() { -self } else { self }
    }
    fn fmt_quaternion_coord(self) -> String {
        format!("{:.064}", &self)
    }
}
