//! See <https://bevyengine.org/examples/camera/first-person-view-model/>

use std::{f32::consts::FRAC_PI_2, iter};

use avian_pickup::prelude::*;
use avian3d::prelude::*;
use bevy::{
    pbr::NotShadowCaster,
    prelude::*,
    render::view::{NoFrustumCulling, RenderLayers},
    scene::SceneInstanceReady,
};
use bevy_enhanced_input::prelude::*;

use crate::{
    gameplay::animation::{AnimationPlayerAncestor, AnimationPlayerLink},
    screens::Screen,
    third_party::avian3d::CollisionLayer,
};

use super::{Player, assets::PlayerAssets, default_input::Rotate};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(spawn_view_model);
    app.add_observer(add_render_layers_to_point_light);
    app.add_observer(rotate_camera_yaw_and_pitch.param_warn_once());
    app.add_systems(
        Update,
        sync_camera_translation_with_player
            .param_warn_once()
            .run_if(in_state(Screen::Gameplay)),
    );
    app.register_type::<PlayerCameraParent>();
    app.register_type::<WorldModelCamera>();
    app.register_type::<CameraSensitivity>();
}

#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
#[require(Transform, Visibility)]
pub(crate) struct PlayerCameraParent;

#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
struct WorldModelCamera;

/// Used implicitly by all entities without a `RenderLayers` component.
/// Our world model camera and all objects other than the player are on this layer.
/// The light source belongs to both layers.
const DEFAULT_RENDER_LAYER: usize = 0;

/// Used by the view model camera and the player's arm.
/// The light source belongs to both layers.
const VIEW_MODEL_RENDER_LAYER: usize = 1;

#[derive(Debug, Component, Reflect, Deref, DerefMut)]
#[reflect(Component)]
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
    assets: Res<PlayerAssets>,
) {
    commands
        .spawn((
            Name::new("PlayerCameraParent"),
            PlayerCameraParent,
            CameraSensitivity::default(),
            StateScoped(Screen::Gameplay),
            AvianPickupActor {
                prop_filter: SpatialQueryFilter::from_mask(CollisionLayer::Prop),
                obstacle_filter: SpatialQueryFilter::from_mask(CollisionLayer::Default),
                actor_filter: SpatialQueryFilter::from_mask(CollisionLayer::Player),
                interaction_distance: 2.0,
                pull: AvianPickupActorPullConfig {
                    impulse: 20.0,
                    ..default()
                },
                hold: AvianPickupActorHoldConfig {
                    distance_to_allow_holding: 2.0,
                    linear_velocity_easing: 0.8,
                    preferred_distance: 0.65,
                    min_distance: 0.4,
                    ..default()
                },
                ..default()
            },
            AnimationPlayerAncestor,
            SpatialListener::new(0.4),
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
                    // We use whatever FOV we set in the animation software, e.g. Blender.
                    // Tip: if you want to set a camera in Blender to the same defaults as Bevy,
                    // see [this issue](https://github.com/kaosat-dev/Blenvy/issues/223)
                    fov: 62.0_f32.to_radians(),
                    ..default()
                }),
                // Only render objects belonging to the view model.
                RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
            ));

            // Spawn the player's right arm.
            parent
                .spawn((Name::new("PlayerArm"), SceneRoot(assets.model.clone())))
                .observe(configure_player_view_model);
        })
        .observe(add_anim_player_link_to_player);
}

fn add_anim_player_link_to_player(
    trigger: Trigger<OnAdd, AnimationPlayerLink>,
    q_anim_player: Query<&AnimationPlayerLink>,
    player: Single<Entity, With<Player>>,
    mut commands: Commands,
) {
    let anim_player_link = q_anim_player.get(trigger.entity()).unwrap();
    commands.entity(*player).insert(*anim_player_link);
}

fn configure_player_view_model(
    trigger: Trigger<SceneInstanceReady>,
    mut commands: Commands,
    q_children: Query<&Children>,
    q_mesh: Query<(), With<Mesh3d>>,
) {
    let view_model = trigger.entity();

    for child in iter::once(view_model)
        .chain(q_children.iter_descendants(view_model))
        .filter(|e| q_mesh.contains(*e))
    {
        commands.entity(child).insert((
            // Ensure the arm is only rendered by the view model camera.
            RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
            // The arm is free-floating, so shadows would look weird.
            NotShadowCaster,
            // The arm's origin is at the origin of the camera, so there is a high risk
            // of it being culled. We want the view model to be visible at all times,
            // so we disable frustum culling.
            NoFrustumCulling,
        ));
    }
}

fn rotate_camera_yaw_and_pitch(
    trigger: Trigger<Fired<Rotate>>,
    mut transform: Single<&mut Transform, With<PlayerCameraParent>>,
) {
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

fn sync_camera_translation_with_player(
    mut player_camera_parent: Single<&mut Transform, With<PlayerCameraParent>>,
    player: Single<&Transform, (With<Player>, Without<PlayerCameraParent>)>,
) {
    player_camera_parent.translation = player.translation;
}

fn add_render_layers_to_point_light(trigger: Trigger<OnAdd, PointLight>, mut commands: Commands) {
    let entity = trigger.entity();
    commands.entity(entity).insert(RenderLayers::from_layers(&[
        DEFAULT_RENDER_LAYER,
        VIEW_MODEL_RENDER_LAYER,
    ]));
}
