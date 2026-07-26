// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Copyright (c) 2026 c1ph3rC4t

//! Quaterlib
//!
//! A quaterion and rotation library for Rust.

#![cfg_attr(feature = "f128", feature(f128))]
mod euler_angles;
mod float_traits;
mod quaternion;

pub use euler_angles::*;
pub use float_traits::*;
pub use quaternion::*;
