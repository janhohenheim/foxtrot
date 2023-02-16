use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, PartialEq, Reflect, FromReflect, TypeUuid, Serialize, Deserialize, Default,
)]
#[reflect(Serialize, Deserialize)]
#[uuid = "93a7c64b-4d6e-4420-b8c1-dfca481d9387"]
pub struct GameConfig {
    pub camera: Camera,
}

#[derive(Debug, Clone, PartialEq, Reflect, FromReflect, Serialize, Deserialize, Default)]
#[reflect(Serialize, Deserialize)]
pub struct Camera {
    fixed_angle: FixedAngle,
}

#[derive(Debug, Clone, PartialEq, Reflect, FromReflect, Serialize, Deserialize)]
#[reflect(Serialize, Deserialize)]
pub struct FixedAngle {
    pub min_distance: f32,
    pub max_distance: f32,
    pub zoom_speed: f32,
    pub rotation_smoothing: f32,
    pub translation_smoothing: f32,
}

impl Default for FixedAngle {
    fn default() -> Self {
        Self {
            min_distance: 5.0,
            max_distance: 20.0,
            zoom_speed: 0.5,
            rotation_smoothing: 45.0,
            translation_smoothing: 50.0,
        }
    }
}
