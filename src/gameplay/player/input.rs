//! Input handling for the player.

use std::any::TypeId;

use bevy::{
    ecs::{lifecycle::HookContext, world::DeferredWorld},
    platform::collections::HashSet,
    prelude::*,
};
use bevy_ahoy::prelude::*;
use bevy_enhanced_input::prelude::{Press, *};

use super::Player;

pub(super) fn plugin(app: &mut App) {
    app.add_input_context::<PlayerInputContext>();

    app.init_resource::<BlocksInput>();
    app.add_systems(
        PreUpdate,
        update_player_input_binding.run_if(resource_changed::<BlocksInput>),
    );
}

#[derive(Debug, InputAction)]
#[action_output(bool)]
pub(crate) struct Interact;

#[derive(Debug, Component, Default)]
#[component(on_add = PlayerInputContext::on_add)]
pub(crate) struct PlayerInputContext;

#[derive(Resource, Default, Reflect, Deref, DerefMut)]
#[reflect(Resource)]
pub(crate) struct BlocksInput(HashSet<TypeId>);

impl PlayerInputContext {
    fn on_add(mut world: DeferredWorld, ctx: HookContext) {
        world
            .commands()
            .entity(ctx.entity)
            .insert(actions!(PlayerInputContext[
                (
                    Action::<Movement>::new(),
                    ActionSettings { consume_input: false, ..default() },
                    DeadZone::default(),
                    Bindings::spawn((
                        Cardinal::wasd_keys(),
                        Axial::left_stick()
                    ))
                ),
                (
                    Action::<Jump>::new(),
                    ActionSettings { consume_input: false, ..default() },
                    Press::default(),
                    bindings![
                        KeyCode::Space,
                        GamepadButton::South,
                    ],
                ),
                (
                    Action::<Tac>::new(),
                    ActionSettings { consume_input: false, ..default() },
                    Press::default(),
                    bindings![
                        KeyCode::Space,
                        GamepadButton::South,
                    ],
                ),
                (
                    Action::<Crane>::new(),
                    ActionSettings { consume_input: false, ..default() },
                    Press::default(),
                    bindings![
                        KeyCode::Space,
                        GamepadButton::South,
                    ],
                ),
                (
                    Action::<Mantle>::new(),
                    ActionSettings { consume_input: false, ..default() },
                    Hold::new(0.2),
                    bindings![
                        KeyCode::Space,
                        GamepadButton::South,
                    ],
                ),
                (
                    Action::<Climbdown>::new(),
                    ActionSettings { consume_input: false, ..default() },
                    bindings![KeyCode::ControlLeft, GamepadButton::LeftTrigger2],
                ),
                (
                    Action::<Crouch>::new(),
                    ActionSettings { consume_input: false, ..default() },
                    bindings![KeyCode::ControlLeft, GamepadButton::LeftTrigger2],
                ),
                (
                    Action::<SwimUp>::new(),
                    ActionSettings { consume_input: false, ..default() },
                    bindings![KeyCode::Space, GamepadButton::South],
                ),
                (
                    Action::<PullObject>::new(),
                    ActionSettings { consume_input: true, ..default() },
                    Press::default(),
                    bindings![MouseButton::Right],
                ),
                (
                    Action::<DropObject>::new(),
                    ActionSettings { consume_input: true, ..default() },
                    Press::default(),
                    bindings![MouseButton::Right],
                ),
                (
                    Action::<ThrowObject>::new(),
                    ActionSettings { consume_input: true, ..default() },
                    Press::default(),
                    bindings![MouseButton::Left],
                ),
                (
                    Action::<RotateCamera>::new(),
                    ActionSettings { consume_input: false, ..default() },

                    Bindings::spawn((
                        Spawn((Binding::mouse_motion(), Scale::splat(0.07))),
                        Axial::right_stick().with((Scale::splat(4.0),  DeadZone::default())),
                    ))
                ),
                (
                    Action::<Interact>::new(),
                    bindings![KeyCode::KeyE, GamepadButton::South]
                ),
            ]));
    }
}

fn update_player_input_binding(
    player: Single<Entity, With<Player>>,
    blocks_input: Res<BlocksInput>,
    mut commands: Commands,
) {
    if blocks_input.is_empty() {
        commands.entity(*player).insert(PlayerInputContext);
    } else {
        commands
            .entity(*player)
            .remove_with_requires::<PlayerInputContext>()
            .despawn_related::<Actions<PlayerInputContext>>();
    }
}
