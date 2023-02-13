use bevy::prelude::*;
use std::f32::consts::{PI, TAU};

pub fn clamp_pitch(up: Vec3, forward: Vec3, angle: f32) -> f32 {
    const MOST_ACUTE_ALLOWED_FROM_ABOVE: f32 = TAU / 10.;
    const MOST_ACUTE_ALLOWED_FROM_BELOW: f32 = TAU / 7.;

    let angle_to_axis = forward.angle_between(up);
    let (acute_angle_to_axis, most_acute_allowed, sign) = if angle_to_axis > PI / 2. {
        (PI - angle_to_axis, MOST_ACUTE_ALLOWED_FROM_ABOVE, -1.)
    } else {
        (angle_to_axis, MOST_ACUTE_ALLOWED_FROM_BELOW, 1.)
    };
    let new_angle = if acute_angle_to_axis < most_acute_allowed {
        angle - sign * (most_acute_allowed - acute_angle_to_axis)
    } else {
        angle
    };
    if new_angle.abs() < 0.01 {
        0.
    } else {
        new_angle
    }
}
