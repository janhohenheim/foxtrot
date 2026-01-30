//! The cameras for the world and the view model.
//!
//! The code is adapted from <https://bevyengine.org/examples/camera/first-person-view-model/>.
//! See that example for more information.

use std::iter;

use avian_pickup::prelude::*;
use avian3d::prelude::*;
#[cfg(feature = "native")]
use bevy::pbr::ScreenSpaceAmbientOcclusion;
use bevy::{
    anti_alias::{fxaa::Fxaa, taa::TemporalAntiAliasing},
    camera::{Exposure, visibility::RenderLayers},
    core_pipeline::{
        Skybox,
        prepass::{DeferredPrepass, DepthPrepass},
        tonemapping::Tonemapping,
    },
    light::{NotShadowCaster, ShadowFilteringMethod},
    post_process::bloom::Bloom,
    prelude::*,
    render::view::Hdr,
    scene::SceneInstanceReady,
};
use bevy_ahoy::camera::CharacterControllerCameraOf;

use crate::{
    CameraOrder, PostPhysicsAppSystems, RenderLayer,
    gameplay::{
        animation::{AnimationPlayerAncestor, AnimationPlayerOf, AnimationPlayers},
        level::LevelAssets,
    },
    screens::{Screen, loading::LoadingScreen},
    third_party::{avian3d::CollisionLayer, bevy_trenchbroom::LoadTrenchbroomModel as _},
};

use super::Player;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<CameraSensitivity>();
    app.init_resource::<WorldModelFov>();

    app.add_observer(spawn_view_model);
    app.add_observer(add_render_layers_to_point_light);
    app.add_observer(add_render_layers_to_spot_light);
    app.add_observer(add_render_layers_to_directional_light);
    app.add_systems(
        Update,
        update_world_model_fov
            .run_if(resource_changed::<WorldModelFov>)
            .in_set(PostPhysicsAppSystems::Update),
    );
}

/// The parent entity of the player's cameras.
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
#[require(Transform, Visibility)]
pub(crate) struct PlayerCamera;

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
#[require(Transform, Visibility)]
struct WorldModelCamera;

fn spawn_view_model(
    add: On<Add, Player>,
    mut commands: Commands,
    assets: Res<AssetServer>,
    level_assets: Res<LevelAssets>,
    fov: Res<WorldModelFov>,
) {
    use bevy_seedling::spatial::SpatialListener3D;

    let env_map = EnvironmentMapLight {
        diffuse_map: level_assets.env_map_diffuse.clone(),
        specular_map: level_assets.env_map_specular.clone(),
        intensity: 300.0,
        ..default()
    };

    // Optimized for a dark outdoor scene at night
    let exposure = Exposure { ev100: 4.5 };

    // Spawn the player camera
    commands
        .spawn((
            // Enable the optional builtin camera controller
            CharacterControllerCameraOf::new(add.entity),
            Name::new("Player Camera Parent"),
            PlayerCamera,
            DespawnOnExit(Screen::Gameplay),
            DespawnOnExit(LoadingScreen::Shaders),
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
            SpatialListener3D,
        ))
        .with_children(|parent| {
            parent.spawn((
                Name::new("World Model Camera"),
                WorldModelCamera,
                Camera3d::default(),
                Projection::from(PerspectiveProjection {
                    fov: fov.to_radians(),
                    ..default()
                }),
                Camera {
                    order: CameraOrder::World.into(),
                    clear_color: Color::srgb_u8(15, 9, 20).into(),
                    ..default()
                },
                Hdr,
                RenderLayers::from(
                    RenderLayer::DEFAULT | RenderLayer::PARTICLES | RenderLayer::GIZMO3,
                ),
                exposure,
                Tonemapping::TonyMcMapface,
                Bloom::NATURAL,
                Skybox {
                    image: level_assets.env_map_specular.clone(),
                    brightness: 8.0,
                    ..default()
                },
                env_map.clone(),
                (
                    Msaa::Off,
                    TemporalAntiAliasing::default(),
                    ShadowFilteringMethod::Temporal,
                    DeferredPrepass,
                ),
                #[cfg(feature = "native")]
                // See https://github.com/bevyengine/bevy/issues/20459
                ScreenSpaceAmbientOcclusion::default(),
            ));

            // Spawn view model camera.
            parent.spawn((
                Name::new("View Model Camera"),
                Camera3d::default(),
                Camera {
                    // Bump the order to render on top of the world model.
                    order: CameraOrder::ViewModel.into(),
                    ..default()
                },
                Hdr,
                Projection::from(PerspectiveProjection {
                    // We use whatever FOV we set in the animation software, e.g. Blender.
                    // Tip: if you want to set a camera in Blender to the same defaults as Bevy,
                    // see [this issue](https://github.com/kaosat-dev/Blenvy/issues/223)
                    fov: 62.0_f32.to_radians(),
                    ..default()
                }),
                // Only render objects belonging to the view model.
                RenderLayers::from(RenderLayer::VIEW_MODEL),
                exposure,
                Tonemapping::TonyMcMapface,
                (DepthPrepass, Msaa::Off, DeferredPrepass, Fxaa::default()),
                env_map,
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
    add: On<Add, AnimationPlayers>,
    q_anim_player: Query<&AnimationPlayers>,
    player: Single<Entity, With<Player>>,
    mut commands: Commands,
) {
    let anim_players = q_anim_player.get(add.entity).unwrap();
    for anim_player in anim_players.iter() {
        commands
            .entity(anim_player)
            .insert(AnimationPlayerOf(*player));
    }
}

fn configure_player_view_model(
    ready: On<SceneInstanceReady>,
    mut commands: Commands,
    q_children: Query<&Children>,
    q_mesh: Query<(), With<Mesh3d>>,
) {
    let view_model = ready.entity;

    for child in iter::once(view_model)
        .chain(q_children.iter_descendants(view_model))
        .filter(|e| q_mesh.contains(*e))
    {
        commands.entity(child).insert((
            // Ensure the arm is only rendered by the view model camera.
            RenderLayers::from(RenderLayer::VIEW_MODEL),
            // The arm is free-floating, so shadows would look weird.
            NotShadowCaster,
        ));
    }
}

fn add_render_layers_to_point_light(add: On<Add, PointLight>, mut commands: Commands) {
    let entity = add.entity;
    commands.entity(entity).insert(RenderLayers::from(
        RenderLayer::DEFAULT | RenderLayer::VIEW_MODEL,
    ));
}

fn add_render_layers_to_spot_light(add: On<Add, SpotLight>, mut commands: Commands) {
    let entity = add.entity;
    commands.entity(entity).insert(RenderLayers::from(
        RenderLayer::DEFAULT | RenderLayer::VIEW_MODEL,
    ));
}

fn add_render_layers_to_directional_light(add: On<Add, DirectionalLight>, mut commands: Commands) {
    let entity = add.entity;
    commands.entity(entity).insert(RenderLayers::from(
        RenderLayer::DEFAULT | RenderLayer::VIEW_MODEL,
    ));
}

#[derive(Resource, Reflect, Debug, Deref, DerefMut)]
#[reflect(Resource)]
pub(crate) struct WorldModelFov(pub(crate) f32);

impl Default for WorldModelFov {
    fn default() -> Self {
        Self(75.0)
    }
}

fn update_world_model_fov(
    projection: Single<&mut Projection, With<WorldModelCamera>>,
    fov: Res<WorldModelFov>,
) {
    let Projection::Perspective(ref mut perspective) = *projection.into_inner() else {
        return;
    };
    perspective.fov = fov.to_radians();
}

#[derive(Resource, Reflect, Debug, Deref, DerefMut)]
#[reflect(Resource)]
pub(crate) struct CameraSensitivity(pub(crate) Vec2);

impl Default for CameraSensitivity {
    fn default() -> Self {
        Self(Vec2::splat(1.0))
    }
}
