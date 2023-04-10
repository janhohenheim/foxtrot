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
pub(crate) struct GameConfig {
    pub(crate) camera: Camera,
    pub(crate) characters: Characters,
    pub(crate) player: Player,
    pub(crate) dialog: Dialog,
}

#[derive(Debug, Clone, PartialEq, Reflect, FromReflect, Serialize, Deserialize, Default)]
#[reflect(Serialize, Deserialize)]
pub(crate) struct Camera {
    pub(crate) fixed_angle: FixedAngle,
    pub(crate) first_person: FirstPerson,
    pub(crate) third_person: ThirdPerson,
    pub(crate) mouse_sensitivity_x: f32,
    pub(crate) mouse_sensitivity_y: f32,
}

#[derive(Debug, Clone, PartialEq, Reflect, FromReflect, Serialize, Deserialize, Default)]
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

#[derive(Debug, Clone, PartialEq, Reflect, FromReflect, Serialize, Deserialize, Default)]
#[reflect(Serialize, Deserialize)]
pub(crate) struct FirstPerson {
    pub(crate) translation_smoothing: f32,
    pub(crate) rotation_smoothing: f32,
    pub(crate) max_pitch: f32,
    pub(crate) min_pitch: f32,
    pub(crate) tracking_smoothing: f32,
}

#[derive(Debug, Clone, PartialEq, Reflect, FromReflect, Serialize, Deserialize, Default)]
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

#[derive(Debug, Clone, PartialEq, Reflect, FromReflect, Serialize, Deserialize, Default)]
#[reflect(Serialize, Deserialize)]
pub(crate) struct Characters {
    pub(crate) model_sync_smoothing: f32,
    pub(crate) rotation_smoothing: f32,
}

#[derive(Debug, Clone, PartialEq, Reflect, FromReflect, Serialize, Deserialize, Default)]
#[reflect(Serialize, Deserialize)]
pub(crate) struct Player {
    pub(crate) rotate_to_speaker_smoothness: f32,
    pub(crate) sprint_effect_speed_threshold: f32,
    pub(crate) fov_saturation_speed: f32,
    pub(crate) min_fov: f32,
    pub(crate) max_fov: f32,
}

#[derive(Debug, Clone, PartialEq, Reflect, FromReflect, Serialize, Deserialize, Default)]
#[reflect(Serialize, Deserialize)]
pub(crate) struct Dialog {
    pub(crate) base_letters_per_second: f32,
}
