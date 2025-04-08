//! See <https://bevyengine.org/examples/camera/first-person-view-model/>

use std::f32::consts::FRAC_PI_2;

use bevy::{
    color::palettes::tailwind, pbr::NotShadowCaster, prelude::*, render::view::RenderLayers,
};
use bevy_enhanced_input::prelude::*;

use crate::screens::Screen;

use super::{Player, input::Rotate};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(spawn_view_model);
    app.add_observer(add_render_layers_to_point_light);
    app.add_observer(rotate_player);
    app.add_systems(Update, sync_with_player);
}

#[derive(Debug, Component)]
#[require(Transform, Visibility)]
pub(crate) struct PlayerCameraParent;

#[derive(Debug, Component)]
struct WorldModelCamera;

/// Used implicitly by all entities without a `RenderLayers` component.
/// Our world model camera and all objects other than the player are on this layer.
/// The light source belongs to both layers.
pub(crate) const DEFAULT_RENDER_LAYER: usize = 0;

/// Used by the view model camera and the player's arm.
/// The light source belongs to both layers.
pub(crate) const VIEW_MODEL_RENDER_LAYER: usize = 1;

#[derive(Debug, Component, Deref, DerefMut)]
pub(crate) struct CameraSensitivity(Vec2);

impl Default for CameraSensitivity {
    fn default() -> Self {
        Self(
            // These factors are just arbitrary mouse sensitivity values.
            // It's often nicer to have a faster horizontal sensitivity than vertical.
            // We use a component for them so that we can make them user-configurable at runtime
            // for accessibility reasons.
            // It also allows you to inspect them in an editor if you `Reflect` the component.
            Vec2::new(0.003, 0.002),
        )
    }
}

fn spawn_view_model(
    _trigger: Trigger<OnAdd, Player>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let arm = meshes.add(Cuboid::new(0.1, 0.1, 0.5));
    let arm_material = materials.add(Color::from(tailwind::TEAL_200));

    commands
        .spawn((
            Name::new("PlayerCameraParent"),
            PlayerCameraParent,
            CameraSensitivity::default(),
            StateScoped(Screen::Gameplay),
        ))
        .with_children(|parent| {
            parent.spawn((
                Name::new("WorldModelCamera"),
                WorldModelCamera,
                Camera3d::default(),
                Projection::from(PerspectiveProjection {
                    fov: 90.0_f32.to_radians(),
                    ..default()
                }),
            ));

            // Spawn view model camera.
            parent.spawn((
                Name::new("ViewModelCamera"),
                Camera3d::default(),
                Camera {
                    // Bump the order to render on top of the world model.
                    order: 1,
                    ..default()
                },
                Projection::from(PerspectiveProjection {
                    fov: 70.0_f32.to_radians(),
                    ..default()
                }),
                // Only render objects belonging to the view model.
                RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
            ));

            // Spawn the player's right arm.
            parent.spawn((
                Name::new("PlayerArm"),
                Mesh3d(arm),
                MeshMaterial3d(arm_material),
                Transform::from_xyz(0.2, -0.1, -0.25),
                // Ensure the arm is only rendered by the view model camera.
                RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
                // The arm is free-floating, so shadows would look weird.
                NotShadowCaster,
            ));
        });
}

fn rotate_player(
    trigger: Trigger<Fired<Rotate>>,
    player: Option<Single<&mut Transform, With<PlayerCameraParent>>>,
) {
    let Some(mut transform) = player else {
        return;
    };
    let delta = trigger.value;

    if delta != Vec2::ZERO {
        // Note that we are not multiplying by delta_time here.
        // The reason is that for mouse movement, we already get the full movement that happened since the last frame.
        // This means that if we multiply by delta_time, we will get a smaller rotation than intended by the user.
        // This situation is reversed when reading e.g. analog input from a gamepad however, where the same rules
        // as for keyboard input apply. Such an input should be multiplied by delta_time to get the intended rotation
        // independent of the framerate.
        let delta_yaw = delta.x;
        let delta_pitch = delta.y;

        let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);
        let yaw = yaw + delta_yaw;

        // If the pitch was ±¹⁄₂ π, the camera would look straight up or down.
        // When the user wants to move the camera back to the horizon, which way should the camera face?
        // The camera has no way of knowing what direction was "forward" before landing in that extreme position,
        // so the direction picked will for all intents and purposes be arbitrary.
        // Another issue is that for mathematical reasons, the yaw will effectively be flipped when the pitch is at the extremes.
        // To not run into these issues, we clamp the pitch to a safe range.
        const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
        let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);

        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
    }
}

fn sync_with_player(
    player_camera_parent: Option<Single<&mut Transform, With<PlayerCameraParent>>>,
    player: Option<Single<&Transform, (With<Player>, Without<PlayerCameraParent>)>>,
) {
    if let Some(mut player_camera_parent) = player_camera_parent {
        if let Some(player) = player {
            player_camera_parent.translation = player.translation;
        }
    }
}

fn add_render_layers_to_point_light(trigger: Trigger<OnAdd, PointLight>, mut commands: Commands) {
    let entity = trigger.entity();
    commands.entity(entity).insert(RenderLayers::from_layers(&[
        DEFAULT_RENDER_LAYER,
        VIEW_MODEL_RENDER_LAYER,
    ]));
}
