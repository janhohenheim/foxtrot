//! Plugin handling the player movement in particular.
//!
//! Note that this is separate from the `movement` module as that could be used
//! for other characters as well.

use animation::{PlayerAnimationState, setup_player_animations};
use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_ahoy::prelude::*;
use bevy_landmass::{Character, prelude::*};

use bevy_trenchbroom::prelude::*;
use input::PlayerInputContext;
use navmesh_position::LastValidPlayerNavmeshPosition;

use crate::{
    animation::AnimationState,
    asset_tracking::LoadResource,
    third_party::{avian3d::CollisionLayer, bevy_trenchbroom::GetTrenchbroomModelPath as _},
};

mod animation;
pub(crate) mod assets;
pub(crate) mod camera;
pub(crate) mod dialogue;
pub(crate) mod input;
pub(crate) mod movement_sound;
pub(crate) mod navmesh_position;
pub(crate) mod pickup;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        animation::plugin,
        assets::plugin,
        camera::plugin,
        input::plugin,
        dialogue::plugin,
        movement_sound::plugin,
        pickup::plugin,
        navmesh_position::plugin,
    ));
    app.add_observer(setup_player);
    app.load_asset::<Gltf>(Player::model_path());
    app.add_systems(PreUpdate, assert_only_one_player);
}

#[point_class(
    base(Transform, Visibility),
    model("models/view_model/view_model.gltf")
)]
pub(crate) struct Player;

/// The radius of the player character's capsule.
pub(crate) const PLAYER_RADIUS: f32 = 0.5;
const PLAYER_HEIGHT: f32 = 1.8;

/// The half height of the player character's capsule is the distance between the character's center and the lowest point of its collider.
const PLAYER_HALF_HEIGHT: f32 = PLAYER_HEIGHT / 2.0;

/// The height used for the player's floating character controller.
///
/// Such a controller works by keeping the character itself at a more-or-less constant height above the ground by
/// using a spring. It's important to make sure that this floating height is greater (even if by little) than the half height.
///
/// In this case, we use 30 cm of padding to make the player float nicely up stairs.
const PLAYER_FLOAT_HEIGHT: f32 = PLAYER_HALF_HEIGHT + 0.01;

fn setup_player(
    add: On<Add, Player>,
    mut commands: Commands,
    archipelago: Single<Entity, With<Archipelago3d>>,
) {
    commands
        .entity(add.entity)
        .insert((
            RigidBody::Kinematic,
            PlayerInputContext,
            // The player character needs to be configured as a dynamic rigid body of the physics
            // engine.
            Collider::cylinder(PLAYER_RADIUS, PLAYER_HEIGHT),
            // This is Tnua's interface component.
            CharacterController::default(),
            ColliderDensity(1_000.0),
            CollisionLayers::new(CollisionLayer::Character, LayerMask::ALL),
            AnimationState::<PlayerAnimationState>::default(),
            children![(
                Name::new("Player Landmass Character"),
                Transform::from_xyz(0.0, -PLAYER_FLOAT_HEIGHT, 0.0),
                Character3dBundle {
                    character: Character::default(),
                    settings: CharacterSettings {
                        radius: PLAYER_RADIUS,
                    },
                    archipelago_ref: ArchipelagoRef3d::new(*archipelago),
                },
                LastValidPlayerNavmeshPosition::default(),
            )],
        ))
        .observe(setup_player_animations);
}

fn assert_only_one_player(player: Populated<(), With<Player>>) {
    assert_eq!(1, player.iter().count());
}
