pub(crate) mod criteria;
pub(crate) mod trait_extension;

pub(crate) fn smoothness_to_lerp_factor(smoothness: f32, dt: f32) -> f32 {
    // Taken from https://github.com/h3r2tic/dolly/blob/main/src/util.rs#L34
    const SMOOTHNESS_MULTIPLIER: f32 = 8.0;
    1.0 - (-SMOOTHNESS_MULTIPLIER * dt / smoothness.max(1e-5)).exp()
}
