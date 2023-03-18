use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Clone,
    PartialEq,
    Reflect,
    FromReflect,
    TypeUuid,
    Serialize,
    Deserialize,
    Default,
    Resource,
)]
#[reflect(Serialize, Deserialize, Resource)]
#[uuid = "93a7c64b-4d6e-4420-b8c1-dfca481d9387"]
pub struct GameConfig {
    pub camera: Camera,
}

#[derive(Debug, Clone, PartialEq, Reflect, FromReflect, Serialize, Deserialize)]
#[reflect(Serialize, Deserialize)]
pub struct Camera {
    pub fixed_angle: FixedAngle,
    pub first_person: FirstPerson,
    pub third_person: ThirdPerson,
    pub mouse_sensitivity_x: f32,
    pub mouse_sensitivity_y: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            fixed_angle: FixedAngle::default(),
            first_person: FirstPerson::default(),
            third_person: ThirdPerson::default(),
            mouse_sensitivity_x: 8e-4,
            mouse_sensitivity_y: 5e-4,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Reflect, FromReflect, Serialize, Deserialize)]
#[reflect(Serialize, Deserialize)]
pub struct FixedAngle {
    pub min_distance: f32,
    pub max_distance: f32,
    pub zoom_speed: f32,
    pub rotation_smoothing: f32,
    pub translation_smoothing: f32,
    pub zoom_in_smoothing: f32,
    pub zoom_out_smoothing: f32,
}

impl Default for FixedAngle {
    fn default() -> Self {
        Self {
            min_distance: 5.0,
            max_distance: 20.0,
            zoom_speed: 0.7,
            rotation_smoothing: 45.0,
            translation_smoothing: 50.0,
            zoom_in_smoothing: 0.3,
            zoom_out_smoothing: 1.2,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Reflect, FromReflect, Serialize, Deserialize)]
#[reflect(Serialize, Deserialize)]
pub struct FirstPerson {
    pub translation_smoothing: f32,
    pub rotation_smoothing: f32,
    pub max_pitch: f32,
    pub min_pitch: f32,
    pub tracking_smoothing: f32,
}

impl Default for FirstPerson {
    fn default() -> Self {
        Self {
            translation_smoothing: 50.0,
            rotation_smoothing: 45.0,
            max_pitch: 36.,
            min_pitch: 50.,
            tracking_smoothing: 0.1,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Reflect, FromReflect, Serialize, Deserialize)]
#[reflect(Serialize, Deserialize)]
pub struct ThirdPerson {
    pub translation_smoothing: f32,
    pub rotation_smoothing: f32,
    pub max_pitch: f32,
    pub min_pitch: f32,
    pub min_distance: f32,
    pub max_distance: f32,
    pub zoom_speed: f32,
    pub min_distance_to_objects: f32,
    pub tracking_smoothing: f32,
    pub zoom_in_smoothing: f32,
    pub zoom_out_smoothing: f32,
}

impl Default for ThirdPerson {
    fn default() -> Self {
        Self {
            translation_smoothing: 0.3,
            rotation_smoothing: 40.3,
            max_pitch: 36.,
            min_pitch: 50.,
            min_distance: 1e-2,
            max_distance: 10.0,
            zoom_speed: 0.7,
            min_distance_to_objects: 5e-1,
            tracking_smoothing: 0.1,
            zoom_in_smoothing: 0.3,
            zoom_out_smoothing: 1.2,
        }
    }
}
