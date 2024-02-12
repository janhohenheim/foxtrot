use bevy::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Reflect, Asset, Serialize, Deserialize, Default, Resource)]
#[reflect(Serialize, Deserialize, Resource)]
pub(crate) struct GameConfig {
    pub(crate) camera: Camera,
    pub(crate) player: PlayerEffects,
}

#[derive(Debug, Clone, PartialEq, Reflect, Serialize, Deserialize, Default)]
#[reflect(Serialize, Deserialize)]
pub(crate) struct Camera {
    pub(crate) fixed_angle: FixedAngle,
    pub(crate) first_person: FirstPerson,
    pub(crate) third_person: ThirdPerson,
    pub(crate) mouse_sensitivity_x: f32,
    pub(crate) mouse_sensitivity_y: f32,
}

#[derive(Debug, Clone, PartialEq, Reflect, Serialize, Deserialize, Default)]
#[reflect(Serialize, Deserialize)]
pub(crate) struct FixedAngle {
    pub(crate) min_distance: f32,
    pub(crate) max_distance: f32,
    pub(crate) zoom_speed: f32,
    pub(crate) rotation_smoothing: f32,
    pub(crate) translation_smoothing: f32,
    pub(crate) zoom_in_smoothing: f32,
    pub(crate) zoom_out_smoothing: f32,
    pub(crate) pitch: f32,
}

#[derive(Debug, Clone, PartialEq, Reflect, Serialize, Deserialize, Default)]
#[reflect(Serialize, Deserialize)]
pub(crate) struct FirstPerson {
    pub(crate) translation_smoothing: f32,
    pub(crate) rotation_smoothing: f32,
    pub(crate) max_pitch: f32,
    pub(crate) min_pitch: f32,
    pub(crate) tracking_smoothing: f32,
}

#[derive(Debug, Clone, PartialEq, Reflect, Serialize, Deserialize, Default)]
#[reflect(Serialize, Deserialize)]
pub(crate) struct ThirdPerson {
    pub(crate) translation_smoothing: f32,
    pub(crate) rotation_smoothing: f32,
    pub(crate) max_pitch: f32,
    pub(crate) min_pitch: f32,
    pub(crate) min_distance: f32,
    pub(crate) max_distance: f32,
    pub(crate) zoom_speed: f32,
    pub(crate) min_distance_to_objects: f32,
    pub(crate) tracking_smoothing: f32,
    pub(crate) zoom_in_smoothing: f32,
    pub(crate) zoom_out_smoothing: f32,
}

#[derive(Debug, Clone, PartialEq, Reflect, Serialize, Deserialize, Default)]
#[reflect(Serialize, Deserialize)]
pub(crate) struct PlayerEffects {
    pub(crate) sprint_effect_speed_threshold: f32,
}
