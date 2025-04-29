//! Plugin handling the player movement in particular.
//!
//! Note that this is separate from the `movement` module as that could be used
//! for other characters as well.

use animation::{PlayerAnimationState, setup_player_animations};
use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;
use bevy_landmass::{Character, prelude::*};
use bevy_tnua::{TnuaAnimatingState, prelude::*};
use bevy_tnua_avian3d::TnuaAvian3dSensorShape;
use bevy_trenchbroom::prelude::*;
use default_input::DefaultInputContext;

use crate::third_party::avian3d::CollisionLayer;

mod animation;
pub(crate) mod assets;
pub(crate) mod camera;
pub(crate) mod default_input;
pub(crate) mod dialogue;
pub(crate) mod movement;
pub(crate) mod movement_sound;
pub(crate) mod pickup;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Player>();
    app.add_plugins((
        animation::plugin,
        assets::plugin,
        camera::plugin,
        default_input::plugin,
        dialogue::plugin,
        movement::plugin,
        movement_sound::plugin,
        pickup::plugin,
    ));
    app.add_observer(setup_player);
}

#[derive(PointClass, Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
#[base(Transform, Visibility)]
#[model("models/view_model/view_model.gltf")]
#[spawn_hook(preload_model::<Self>)]
// In Wasm, TrenchBroom classes are not automatically registered.
// So, we need to manually register the class in `src/third_party/bevy_trenchbroom/mod.rs`.
pub(crate) struct Player;

/// The radius of the player character's capsule.
pub(crate) const PLAYER_RADIUS: f32 = 0.5;
/// The length of the player character's capsule. Note that
const PLAYER_CAPSULE_LENGTH: f32 = 1.0;

/// The total height of the player character's capsule. A capsule's height is `length + 2 * radius`.
const PLAYER_HEIGHT: f32 = PLAYER_CAPSULE_LENGTH + 2.0 * PLAYER_RADIUS;
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
    trigger: Trigger<OnAdd, Player>,
    mut commands: Commands,
    archipelago: Single<Entity, With<Archipelago3d>>,
) {
    let player_character = commands
        .spawn((
            Name::new("Player Landmass Character"),
            Transform::from_xyz(0.0, -PLAYER_FLOAT_HEIGHT, 0.0),
            Character3dBundle {
                character: Character::default(),
                settings: CharacterSettings {
                    radius: PLAYER_RADIUS,
                },
                archipelago_ref: ArchipelagoRef3d::new(*archipelago),
            },
            ChildOf(trigger.target()),
        ))
        .id();

    commands
        .entity(trigger.target())
        .insert((
            RigidBody::Dynamic,
            Actions::<DefaultInputContext>::default(),
            // The player character needs to be configured as a dynamic rigid body of the physics
            // engine.
            Collider::capsule(PLAYER_RADIUS, PLAYER_CAPSULE_LENGTH),
            // This is Tnua's interface component.
            TnuaController::default(),
            // A sensor shape is not strictly necessary, but without it we'll get weird results.
            TnuaAvian3dSensorShape(Collider::cylinder(PLAYER_RADIUS - 0.01, 0.0)),
            // Tnua can fix the rotation, but the character will still get rotated before it can do so.
            // By locking the rotation we can prevent this.
            LockedAxes::ROTATION_LOCKED,
            // Movement feels nicer without friction.
            Friction {
                dynamic_coefficient: 0.0,
                static_coefficient: 0.0,
                combine_rule: CoefficientCombine::Multiply,
            },
            ColliderDensity(100.0),
            TransformInterpolation,
            CollisionLayers::new(CollisionLayer::Character, LayerMask::ALL),
            TnuaAnimatingState::<PlayerAnimationState>::default(),
            PlayerLandmassCharacter(player_character),
        ))
        .observe(setup_player_animations);
}

#[derive(Component)]
pub(crate) struct PlayerLandmassCharacter(pub(crate) Entity);
