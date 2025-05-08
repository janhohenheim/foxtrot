//! The cameras for the world and the view model.
//!
//! The code is adapted from <https://bevyengine.org/examples/camera/first-person-view-model/>.
//! See that example for more information.

use std::{f32::consts::FRAC_PI_2, iter};

use avian_pickup::prelude::*;
use avian3d::prelude::*;
use bevy::{
    core_pipeline::{bloom::Bloom, tonemapping::Tonemapping},
    pbr::NotShadowCaster,
    prelude::*,
    render::{
        camera::Exposure,
        view::{NoFrustumCulling, RenderLayers},
    },
    scene::SceneInstanceReady,
};
use bevy_enhanced_input::prelude::*;

use crate::{
    AppSystems, CameraOrder, RenderLayer,
    gameplay::{
        animation::{AnimationPlayerAncestor, AnimationPlayerOf, AnimationPlayers},
        crosshair::cursor::IsCursorForcedFreed,
    },
    screens::Screen,
    third_party::{avian3d::CollisionLayer, bevy_trenchbroom::LoadTrenchbroomModel as _},
};

use super::{PLAYER_FLOAT_HEIGHT, Player, default_input::Rotate};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(spawn_view_model);
    app.add_observer(add_render_layers_to_point_light);
    app.add_observer(add_render_layers_to_spot_light);
    app.add_observer(add_render_layers_to_directional_light);
    app.add_observer(rotate_camera_yaw_and_pitch);
    app.add_systems(
        Update,
        sync_camera_translation_with_player
            .run_if(in_state(Screen::Gameplay))
            .in_set(AppSystems::Update),
    );
    app.register_type::<PlayerCamera>();
}

/// The parent entity of the player's cameras.
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
#[require(Transform, Visibility)]
pub(crate) struct PlayerCamera;

fn spawn_view_model(
    trigger: Trigger<OnAdd, Player>,
    player_transform: Query<&Transform>,
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    commands.insert_resource(AmbientLight {
        color: Color::srgb_u8(68, 71, 88),
        brightness: 180.0,
        ..default()
    });
    let player_transform = player_transform.get(trigger.target()).unwrap();
    commands
        .spawn((
            Name::new("Player Camera Parent"),
            PlayerCamera,
            *player_transform,
            StateScoped(Screen::Gameplay),
            AvianPickupActor {
                prop_filter: SpatialQueryFilter::from_mask(CollisionLayer::Prop),
                obstacle_filter: SpatialQueryFilter::from_mask(CollisionLayer::Default),
                actor_filter: SpatialQueryFilter::from_mask(CollisionLayer::Character),
                interaction_distance: 2.0,
                pull: AvianPickupActorPullConfig {
                    impulse: 20.0,
                    // We are not limiting ourselves to the mass of props.
                    max_prop_mass: 10_000.0,
                },
                hold: AvianPickupActorHoldConfig {
                    distance_to_allow_holding: 2.0,
                    linear_velocity_easing: 0.7,
                    ..default()
                },
                ..default()
            },
            AnimationPlayerAncestor,
            SpatialListener::new(0.4),
        ))
        .with_children(|parent| {
            parent.spawn((
                Name::new("World Model Camera"),
                Camera3d::default(),
                Camera {
                    order: CameraOrder::World.into(),
                    hdr: true,
                    clear_color: Color::srgb_u8(15, 9, 20).into(),
                    ..default()
                },
                Projection::from(PerspectiveProjection {
                    fov: 75.0_f32.to_radians(),
                    ..default()
                }),
                RenderLayers::from(
                    RenderLayer::DEFAULT | RenderLayer::PARTICLES | RenderLayer::TRANSLUCENT,
                ),
                Exposure::INDOOR,
                Tonemapping::TonyMcMapface,
                Bloom::NATURAL,
            ));

            // Spawn view model camera.
            parent.spawn((
                Name::new("View Model Camera"),
                Camera3d::default(),
                Camera {
                    // Bump the order to render on top of the world model.
                    order: CameraOrder::ViewModel.into(),
                    hdr: true,
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
                RenderLayers::from(RenderLayer::VIEW_MODEL),
                Exposure::INDOOR,
                Tonemapping::TonyMcMapface,
            ));

            // Spawn the player's view model
            parent
                .spawn((
                    Name::new("View Model"),
                    SceneRoot(assets.load_trenchbroom_model::<Player>()),
                ))
                .observe(configure_player_view_model);
        })
        .observe(move_anim_players_relationship_to_player);
}

/// It makes more sense for the animation players to be related to the [`Player`] entity
/// than to the [`PlayerCamera`] entity, so let's move the relationship there.
fn move_anim_players_relationship_to_player(
    trigger: Trigger<OnAdd, AnimationPlayers>,
    q_anim_player: Query<&AnimationPlayers>,
    player: Single<Entity, With<Player>>,
    mut commands: Commands,
) {
    let anim_players = q_anim_player.get(trigger.target()).unwrap();
    for anim_player in anim_players.iter() {
        commands
            .entity(anim_player)
            .insert(AnimationPlayerOf(*player));
    }
}

fn configure_player_view_model(
    trigger: Trigger<SceneInstanceReady>,
    mut commands: Commands,
    q_children: Query<&Children>,
    q_mesh: Query<(), With<Mesh3d>>,
) {
    let view_model = trigger.target();

    for child in iter::once(view_model)
        .chain(q_children.iter_descendants(view_model))
        .filter(|e| q_mesh.contains(*e))
    {
        commands.entity(child).insert((
            // Ensure the arm is only rendered by the view model camera.
            RenderLayers::from(RenderLayer::VIEW_MODEL),
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
    mut transform: Single<&mut Transform, With<PlayerCamera>>,
    cursor_forced_freed: Res<IsCursorForcedFreed>,
) {
    if cursor_forced_freed.0 {
        return;
    }
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
    mut player_camera_parent: Single<&mut Transform, With<PlayerCamera>>,
    player: Single<&Transform, (With<Player>, Without<PlayerCamera>)>,
) {
    let camera_height = 1.84;
    player_camera_parent.translation =
        player.translation + Vec3::Y * (camera_height - PLAYER_FLOAT_HEIGHT);
}

fn add_render_layers_to_point_light(trigger: Trigger<OnAdd, PointLight>, mut commands: Commands) {
    let entity = trigger.target();
    commands.entity(entity).insert(RenderLayers::from(
        RenderLayer::DEFAULT | RenderLayer::VIEW_MODEL,
    ));
}

fn add_render_layers_to_spot_light(trigger: Trigger<OnAdd, SpotLight>, mut commands: Commands) {
    let entity = trigger.target();
    commands.entity(entity).insert(RenderLayers::from(
        RenderLayer::DEFAULT | RenderLayer::VIEW_MODEL,
    ));
}

fn add_render_layers_to_directional_light(
    trigger: Trigger<OnAdd, DirectionalLight>,
    mut commands: Commands,
) {
    let entity = trigger.target();
    commands.entity(entity).insert(RenderLayers::from(
        RenderLayer::DEFAULT | RenderLayer::VIEW_MODEL,
    ));
}
