//! Plugin handling the player character in particular.
//! Note that this is separate from the `movement` module as that could be used
//! for other characters as well.

use avian3d::prelude::*;
use bevy::{
    ecs::{component::ComponentId, world::DeferredWorld},
    prelude::*,
};
use bevy_enhanced_input::prelude::*;
use bevy_trenchbroom::prelude::*;

use crate::{asset_tracking::LoadResource, third_party::bevy_trenchbroom::LoadTrenchbroomModel};

use super::movement::MovementController;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Player>();

    app.register_type::<PlayerAssets>();
    app.load_resource::<PlayerAssets>();

    // Record directional input as movement controls.
    app.add_actions_marker::<Player>() // All contexts should be registered.
        .add_observer(binding) // Add observer to setup bindings.
        .add_observer(apply_movement)
        .add_observer(jump);
}

const DEFAULT_SPEED: f32 = 1.0;

// To define mappings for actions, write an observer for `Binding`.
// It's also possible to create bindings before the insertion,
// but this way you can conveniently reload bindings when settings change.
fn binding(trigger: Trigger<Binding<Player>>, mut players: Query<&mut Actions<Player>>) {
    let mut actions = players.get_mut(trigger.entity()).unwrap();

    // Mappings like WASD or sticks are very common,
    // so we provide built-ins to assign all keys/axes at once.
    // We don't assign any conditions and in this case the action will
    // be triggered with any non-zero value.
    actions
        .bind::<Move>()
        .to((Cardinal::wasd_keys(), GamepadStick::Left))
        .with_modifiers((
            DeadZone::default(), // Apply non-uniform normalization to ensure consistent speed, otherwise diagonal movement will be faster.
            SmoothNudge::default(), // Make movement smooth and independent of the framerate. To only make it framerate-independent, use `DeltaScale`.
            Scale::splat(DEFAULT_SPEED), // Additionally multiply by a constant to achieve the desired speed.
        ));

    // Multiple inputs can be assigned to a single action,
    // and the action will respond to any of them.
    actions
        .bind::<Jump>()
        .to((KeyCode::Space, GamepadButton::South));
}

// All actions should implement the `InputAction` trait.
// It can be done manually, but we provide a derive for convenience.
// The only necessary parameter is `output`, which defines the output type.
#[derive(Debug, InputAction)]
#[input_action(output = Vec2)]
struct Move;

#[derive(Debug, InputAction)]
#[input_action(output = bool)]
struct Jump;

#[derive(
    PointClass, Component, ActionsMarker, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect,
)]
#[reflect(Component)]
#[require(Transform, Visibility)]
#[model("models/suzanne/Suzanne.gltf")]
#[component(on_add = Self::on_add)]
pub(crate) struct Player;

impl Player {
    fn on_add(mut world: DeferredWorld, entity: Entity, _id: ComponentId) {
        let Some(asset_server) = world.get_resource::<AssetServer>() else {
            return;
        };

        let suzanne = asset_server.load_trenchbroom_model::<Self>();

        world.commands().entity(entity).insert((
            SceneRoot(suzanne),
            RigidBody::Dynamic,
            ColliderConstructorHierarchy::new(ColliderConstructor::ConvexDecompositionFromMesh),
            TrenchBroomGltfRotationFix,
            Actions::<Player>::default(),
            MovementController::default(),
        ));
    }
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub(crate) struct PlayerAssets {
    #[dependency]
    model: Handle<Scene>,
    #[dependency]
    pub(crate) steps: Vec<Handle<AudioSource>>,
}

impl FromWorld for PlayerAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            model: assets.load_trenchbroom_model::<Player>(),
            steps: vec![
                assets.load("audio/sound_effects/step1.ogg"),
                assets.load("audio/sound_effects/step2.ogg"),
                assets.load("audio/sound_effects/step3.ogg"),
                assets.load("audio/sound_effects/step4.ogg"),
            ],
        }
    }
}

fn apply_movement(trigger: Trigger<Fired<Move>>, mut players: Query<&mut MovementController>) {
    let mut transform = players.get_mut(trigger.entity()).unwrap();
    // The value has already been preprocessed by defined modifiers.
    transform.intent = trigger.value;
}

fn jump(_trigger: Trigger<Started<Jump>>) {
    info!("jump");
}
