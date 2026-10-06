// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Copyright (c) 2026 c1ph3rC4t

use crate::{Float, Quaternion};

/// A Euler angle struct.
pub struct EulerAngle<T: Float> {
    /// α angle.
    pub alpha: T,
    /// β angle.
    pub beta: T,
    /// ɣ angle.
    pub gamma: T,
}

impl<T: Float> EulerAngle<T> {
    /// Creates a new [`EulerAngle`].
    pub const fn new(alpha: T, beta: T, gamma: T) -> Self {
        Self { alpha, beta, gamma }
    }
}

impl<T: Float> From<Quaternion<T>> for EulerAngle<T> {
    fn from(q: Quaternion<T>) -> Self {
        let q = q.norm();
        let wy_xz2 = T::two() * (q.w.clone() * q.y.clone() - q.x.clone() * q.z.clone());

        Self {
            alpha: T::atan2(
                T::two() * (q.w.clone() * q.x.clone() + q.y.clone() * q.z.clone()),
                T::one() - T::two() * (q.x.clone() * q.x.clone() + q.y.clone() * q.y.clone()),
            ),
            beta: -T::frac_pi_2()
                + T::two()
                    * T::atan2(
                        (T::one() + wy_xz2.clone()).sqrt(),
                        (T::one() - wy_xz2).sqrt(),
                    ),
            gamma: T::atan2(
                T::two() * (q.w.clone() * q.z.clone() + q.x.clone() * q.y.clone()),
                T::one() - T::two() * (q.y.clone() * q.y.clone() + q.z.clone() * q.z),
            ),
        }
    }
}
