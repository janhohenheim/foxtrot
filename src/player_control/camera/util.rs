use bevy::prelude::*;
use std::f32::consts::PI;

pub fn clamp_pitch(
    up: Vec3,
    forward: Vec3,
    angle: f32,
    most_acute_from_above: f32,
    most_acute_from_below: f32,
) -> f32 {
    let angle_to_axis = forward.angle_between(up);
    let (acute_angle_to_axis, most_acute_allowed, sign) = if angle_to_axis > PI / 2. {
        (PI - angle_to_axis, most_acute_from_above, -1.)
    } else {
        (angle_to_axis, most_acute_from_below, 1.)
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
