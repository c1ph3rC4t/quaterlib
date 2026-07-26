// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Copyright (c) 2026 c1ph3rC4t

use crate::{EulerAngle, Float};
use std::{
    fmt,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign},
};

/// A quaternion struct.
#[derive(Clone, Copy)]
pub struct Quaternion<T: Float> {
    /// W coordinate.
    pub w: T,
    /// X coordinate.
    pub x: T,
    /// Y coordinate.
    pub y: T,
    /// Z coordinate.
    pub z: T,
}

impl<T: Float> Quaternion<T> {
    /// Creates a new [`Quaternion`].
    pub const fn new(w: T, x: T, y: T, z: T) -> Self {
        Self { w, x, y, z }
    }

    /// Creates a new rotation [`Quaternion`].
    pub fn new_rot(axis: (T, T, T), alpha: T) -> Self {
        let half_alpha = alpha / T::two();
        let sin_half_alpha = half_alpha.clone().sin();

        Self {
            w: half_alpha.cos(),
            x: axis.0 * sin_half_alpha.clone(),
            y: axis.1 * sin_half_alpha.clone(),
            z: axis.2 * sin_half_alpha,
        }
    }

    /// The length of `self`.
    #[must_use]
    pub fn len(&self) -> T {
        (self.w.clone() * self.w.clone()
            + self.x.clone() * self.x.clone()
            + self.y.clone() * self.y.clone()
            + self.z.clone() * self.z.clone())
        .sqrt()
    }

    /// The squared length of `self`.
    #[must_use]
    pub fn sq_len(&self) -> T {
        self.w.clone() * self.w.clone()
            + self.x.clone() * self.x.clone()
            + self.y.clone() * self.y.clone()
            + self.z.clone() * self.z.clone()
    }

    /// Calculate the normalized version of `self`.
    #[must_use]
    pub fn norm(&self) -> Self {
        let len = self.len();

        Self {
            w: self.w.clone() / len.clone(),
            x: self.x.clone() / len.clone(),
            y: self.y.clone() / len.clone(),
            z: self.z.clone() / len,
        }
    }

    /// Normalize `self`.
    pub fn norm_mut(&mut self) {
        let len = self.len();

        self.w /= len.clone();
        self.x /= len.clone();
        self.y /= len.clone();
        self.z /= len;
    }

    /// Calculate the conjugate of `self`.
    #[must_use]
    pub fn con(&self) -> Self {
        Self {
            w: self.w.clone(),
            x: -self.x.clone(),
            y: -self.y.clone(),
            z: -self.z.clone(),
        }
    }

    /// Replace `self` with the conjugate of `self`.
    pub fn con_mut(&mut self) {
        self.x = -self.x.clone();
        self.y = -self.y.clone();
        self.z = -self.z.clone();
    }

    /// Calculate the inverse of `self`.
    #[must_use]
    pub fn inv(&self) -> Self {
        let sq_len = self.sq_len();
        let q = self.con();

        Self {
            w: q.w / sq_len.clone(),
            x: q.x / sq_len.clone(),
            y: q.y / sq_len.clone(),
            z: q.z / sq_len,
        }
    }

    /// Replace `self` with the inverse of `self`.
    pub fn inv_mut(&mut self) {
        let sq_len = self.sq_len();
        self.con_mut();

        self.w /= sq_len.clone();
        self.x /= sq_len.clone();
        self.y /= sq_len.clone();
        self.z /= sq_len;
    }

    /// Calculate rotating `self` by `rhs`.
    #[must_use]
    pub fn apply_rot(&self, rhs: &Self) -> Self {
        rhs.clone() * self.clone() * rhs.con()
    }

    /// Rotate `self` by `rhs`.
    pub fn apply_rot_mut(&mut self, rhs: &Self) {
        *self = rhs.clone() * self.clone() * rhs.con();
    }

    /// Calculate rotating `rhs` by `self`.
    #[must_use]
    pub fn rot(&self, rhs: Self) -> Self {
        self.clone() * rhs * self.clone().con()
    }

    /// Rotate `rhs` by `self`.
    pub fn rot_mut(&self, rhs: &mut Self) {
        *rhs = self.clone() * rhs.clone() * self.clone().con();
    }
}

impl<T: Float> Add for Quaternion<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            w: self.w + rhs.w,
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: Float> AddAssign for Quaternion<T> {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.clone() + rhs;
    }
}

impl<T: Float> Sub for Quaternion<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            w: self.w - rhs.w,
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T: Float> SubAssign for Quaternion<T> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.clone() - rhs;
    }
}

impl<T: Float> Neg for Quaternion<T> {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            w: -self.w,
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<T: Float> Mul for Quaternion<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            w: self.w.clone() * rhs.w.clone()
                - self.x.clone() * rhs.x.clone()
                - self.y.clone() * rhs.y.clone()
                - self.z.clone() * rhs.z.clone(),
            x: self.w.clone() * rhs.x.clone()
                + self.x.clone() * rhs.w.clone()
                + self.y.clone() * rhs.z.clone()
                - self.z.clone() * rhs.y.clone(),
            y: self.w.clone() * rhs.y.clone() - self.x.clone() * rhs.z.clone()
                + self.y.clone() * rhs.w.clone()
                + self.z.clone() * rhs.x.clone(),
            z: self.w.clone() * rhs.z.clone() + self.x.clone() * rhs.y.clone()
                - self.y.clone() * rhs.x.clone()
                + self.z * rhs.w,
        }
    }
}

impl<T: Float> MulAssign for Quaternion<T> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.clone() * rhs;
    }
}

impl<T: Float> Rem for Quaternion<T> {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self {
        self.rot(rhs)
    }
}

impl<T: Float> RemAssign for Quaternion<T> {
    fn rem_assign(&mut self, rhs: Self) {
        self.apply_rot_mut(&rhs);
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl<T: Float> Div for Quaternion<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        self * rhs.inv()
    }
}

#[allow(clippy::suspicious_op_assign_impl)]
impl<T: Float> DivAssign for Quaternion<T> {
    fn div_assign(&mut self, rhs: Self) {
        *self = self.clone() / rhs;
    }
}

impl<T: Float> From<[T; 4]> for Quaternion<T> {
    fn from(a: [T; 4]) -> Self {
        Self {
            w: a[0].clone(),
            x: a[1].clone(),
            y: a[2].clone(),
            z: a[3].clone(),
        }
    }
}

impl<T: Float> From<Quaternion<T>> for [T; 4] {
    fn from(q: Quaternion<T>) -> Self {
        [q.w, q.x, q.y, q.z]
    }
}

impl<T: Float> From<(T, T, T, T)> for Quaternion<T> {
    fn from((w, x, y, z): (T, T, T, T)) -> Self {
        Self { w, x, y, z }
    }
}

impl<T: Float> From<Quaternion<T>> for (T, T, T, T) {
    fn from(q: Quaternion<T>) -> Self {
        (q.w, q.x, q.y, q.z)
    }
}

impl<T: Float> From<[T; 3]> for Quaternion<T> {
    fn from(a: [T; 3]) -> Self {
        Self {
            w: T::zero(),
            x: a[0].clone(),
            y: a[1].clone(),
            z: a[2].clone(),
        }
    }
}

impl<T: Float> From<Quaternion<T>> for [T; 3] {
    fn from(q: Quaternion<T>) -> Self {
        [q.x, q.y, q.z]
    }
}

impl<T: Float> From<(T, T, T)> for Quaternion<T> {
    fn from((x, y, z): (T, T, T)) -> Self {
        Self {
            w: T::zero(),
            x,
            y,
            z,
        }
    }
}

impl<T: Float> From<Quaternion<T>> for (T, T, T) {
    fn from(q: Quaternion<T>) -> Self {
        (q.x, q.y, q.z)
    }
}

impl<T: Float> From<EulerAngle<T>> for Quaternion<T> {
    fn from(e: EulerAngle<T>) -> Self {
        let half_alpha = e.alpha / T::two();
        let half_beta = e.beta / T::two();
        let half_gamma = e.gamma / T::two();

        let ca = half_alpha.clone().cos();
        let sa = half_alpha.sin();
        let cb = half_beta.clone().cos();
        let sb = half_beta.sin();
        let cg = half_gamma.clone().cos();
        let sg = half_gamma.sin();

        Self {
            w: ca.clone() * cb.clone() * cg.clone() + sa.clone() * sb.clone() * sg.clone(),
            x: sa.clone() * cb.clone() * cg.clone() - ca.clone() * sb.clone() * sg.clone(),
            y: ca.clone() * sb.clone() * cg.clone() + sa.clone() * cb.clone() * sg.clone(),
            z: ca * cb * sg - sa * sb * cg,
        }
    }
}

impl<T: Float> fmt::Display for Quaternion<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({} {} {}i {} {}j {} {}k)<{}>",
            // "({:.03} {} {:.03}i {} {:.03}j {} {:.03}k)",
            self.w.clone().fmt_quaternion_coord(),
            if self.x >= T::zero() { '+' } else { '-' },
            self.x.clone().abs().fmt_quaternion_coord(),
            if self.y >= T::zero() { '+' } else { '-' },
            self.y.clone().abs().fmt_quaternion_coord(),
            if self.z >= T::zero() { '+' } else { '-' },
            self.z.clone().abs().fmt_quaternion_coord(),
            std::any::type_name::<T>()
        )?;
        Ok(())
    }
}

impl<T: Float> fmt::Debug for Quaternion<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::float_cmp)]
    #[test]
    fn test_f32() {
        let q = Quaternion::new(1.0f32, 2.0f32, 3.0f32, 4.0f32);
        assert_eq!(q.w, 1.0f32);
        assert_eq!(q.x, 2.0f32);
        assert_eq!(q.y, 3.0f32);
        assert_eq!(q.z, 4.0f32);
    }

    #[allow(clippy::float_cmp)]
    #[test]
    fn test_f64() {
        let q = Quaternion::new(1.0f64, 2.0f64, 3.0f64, 4.0f64);
        assert_eq!(q.w, 1.0f64);
        assert_eq!(q.x, 2.0f64);
        assert_eq!(q.y, 3.0f64);
        assert_eq!(q.z, 4.0f64);
    }

    #[cfg(feature = "f128")]
    #[allow(clippy::float_cmp)]
    #[test]
    fn test_f128() {
        let q = Quaternion::new(1.0f128, 2.0f128, 3.0f128, 4.0f128);
        assert_eq!(q.w, 1.0f128);
        assert_eq!(q.x, 2.0f128);
        assert_eq!(q.y, 3.0f128);
        assert_eq!(q.z, 4.0f128);
    }

    #[cfg(feature = "fraction")]
    #[test]
    fn test_fraction() {
        use fraction::BigFraction;

        let q = Quaternion::new(
            BigFraction::from(1),
            BigFraction::from(2),
            BigFraction::from(3),
            BigFraction::from(4),
        );
        assert_eq!(q.w, BigFraction::from(1));
        assert_eq!(q.x, BigFraction::from(2));
        assert_eq!(q.y, BigFraction::from(3));
        assert_eq!(q.z, BigFraction::from(4));
    }
}
