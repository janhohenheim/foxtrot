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
    pub characters: Characters,
    pub player: Player,
    pub dialog: Dialog,
}

#[derive(Debug, Clone, PartialEq, Reflect, FromReflect, Serialize, Deserialize, Default)]
#[reflect(Serialize, Deserialize)]
pub struct Camera {
    pub fixed_angle: FixedAngle,
    pub first_person: FirstPerson,
    pub third_person: ThirdPerson,
    pub mouse_sensitivity_x: f32,
    pub mouse_sensitivity_y: f32,
}

#[derive(Debug, Clone, PartialEq, Reflect, FromReflect, Serialize, Deserialize, Default)]
#[reflect(Serialize, Deserialize)]
pub struct FixedAngle {
    pub min_distance: f32,
    pub max_distance: f32,
    pub zoom_speed: f32,
    pub rotation_smoothing: f32,
    pub translation_smoothing: f32,
    pub zoom_in_smoothing: f32,
    pub zoom_out_smoothing: f32,
    pub pitch: f32,
}

#[derive(Debug, Clone, PartialEq, Reflect, FromReflect, Serialize, Deserialize, Default)]
#[reflect(Serialize, Deserialize)]
pub struct FirstPerson {
    pub translation_smoothing: f32,
    pub rotation_smoothing: f32,
    pub max_pitch: f32,
    pub min_pitch: f32,
    pub tracking_smoothing: f32,
}

#[derive(Debug, Clone, PartialEq, Reflect, FromReflect, Serialize, Deserialize, Default)]
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

#[derive(Debug, Clone, PartialEq, Reflect, FromReflect, Serialize, Deserialize, Default)]
#[reflect(Serialize, Deserialize)]
pub struct Characters {
    pub model_sync_smoothing: f32,
    pub rotation_smoothing: f32,
}

#[derive(Debug, Clone, PartialEq, Reflect, FromReflect, Serialize, Deserialize, Default)]
#[reflect(Serialize, Deserialize)]
pub struct Player {
    pub rotate_to_speaker_smoothness: f32,
    pub sprint_effect_speed_threshold: f32,
    pub fov_saturation_speed: f32,
    pub min_fov: f32,
    pub max_fov: f32,
}

#[derive(Debug, Clone, PartialEq, Reflect, FromReflect, Serialize, Deserialize, Default)]
#[reflect(Serialize, Deserialize)]
pub struct Dialog {
    pub base_letters_per_second: f32,
}
