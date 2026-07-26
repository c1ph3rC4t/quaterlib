// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Copyright (c) 2026 c1ph3rC4t

//! Title

#![cfg_attr(feature = "f128", feature(f128))]
#[cfg(feature = "fraction")]
use fraction::BigFraction;
use quaterlib::{Float, Quaternion};
fn main() {
    let mut position = Quaternion::from((1.0f32, 0.0f32, 0.0f32));

    let rotation = Quaternion::new_rot((0.0f32, 1.0f32, 0.0f32), f32::frac_pi_2());

    position %= rotation;
    position %= rotation;
    position %= rotation;

    position %= rotation;
    position %= rotation;
    position %= rotation;

    println!(" pre norm: {position}");

    position.norm_mut();

    println!("post norm: {position}");

    let mut position = Quaternion::from((1.0f64, 0.0f64, 0.0f64));

    let rotation = Quaternion::new_rot((0.0f64, 1.0f64, 0.0f64), f64::frac_pi_2());

    position %= rotation;
    position %= rotation;
    position %= rotation;

    position %= rotation;
    position %= rotation;
    position %= rotation;

    println!(" pre norm: {position}");

    position.norm_mut();

    println!("post norm: {position}");

    #[cfg(feature = "fraction")]
    {
        let mut position = Quaternion::from((
            BigFraction::from(1),
            BigFraction::from(0),
            BigFraction::from(0),
        ));

        let rotation = Quaternion::new_rot(
            (
                BigFraction::from(0),
                BigFraction::from(1),
                BigFraction::from(0),
            ),
            BigFraction::frac_pi_2(),
        );

        position %= rotation.clone();
        position %= rotation.clone();
        position %= rotation.clone();

        position %= rotation.clone();
        position %= rotation.clone();
        position %= rotation;

        println!(" pre norm: {position}");

        position.norm_mut();

        println!("post norm: {position}");
    }

    #[cfg(feature = "f128")]
    {
        let mut position = Quaternion::from((1.0f128, 0.0f128, 0.0f128));

        let rotation = Quaternion::new_rot((0.0f128, 1.0f128, 0.0f128), f128::frac_pi_2());

        position %= rotation;
        position %= rotation;
        position %= rotation;

        position %= rotation;
        position %= rotation;
        position %= rotation;

        println!(" pre norm: {position}");

        position.norm_mut();

        println!("post norm: {position}");
    }
}
