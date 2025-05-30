use std::f32::consts::TAU;

use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;
#[cfg(feature = "hot_patch")]
use bevy_simple_subsecond_system::hot;
use bevy_tnua::prelude::*;

use crate::fixed_update_inspection::did_fixed_update_happen;

use super::default_input::{Jump, Move};

use super::PLAYER_FLOAT_HEIGHT;
use super::{Player, camera::PlayerCamera};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        FixedUpdate,
        apply_movement.in_set(TnuaUserControlsSystemSet),
    );
    app.add_systems(
        Update,
        clear_accumulated_input.run_if(did_fixed_update_happen),
    );
    app.add_observer(jump);
    app.add_observer(accumulate_movement);
    app.add_observer(init_accumulated_input);

    app.register_type::<AccumulatedInput>();
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
struct AccumulatedInput {
    last_move: Option<Vec3>,
}

#[cfg_attr(feature = "hot_patch", hot)]
fn init_accumulated_input(trigger: Trigger<OnAdd, Player>, mut commands: Commands) {
    commands
        .entity(trigger.target())
        .insert(AccumulatedInput::default());
}

#[cfg_attr(feature = "hot_patch", hot)]
fn accumulate_movement(
    trigger: Trigger<Fired<Move>>,
    mut accumulated_inputs: Single<&mut AccumulatedInput>,
) {
    accumulated_inputs.last_move.replace(trigger.value);
}

#[cfg_attr(feature = "hot_patch", hot)]
fn clear_accumulated_input(mut accumulated_inputs: Query<&mut AccumulatedInput>) {
    for mut accumulated_input in &mut accumulated_inputs {
        accumulated_input.last_move = None;
    }
}

#[cfg_attr(feature = "hot_patch", hot)]
fn apply_movement(
    player_controller: Single<(&mut TnuaController, &AccumulatedInput)>,
    transform: Single<&Transform, With<PlayerCamera>>,
) {
    let (mut controller, accumulated_input) = player_controller.into_inner();
    let last_move = accumulated_input.last_move.unwrap_or_default();
    // Feed the basis every frame. Even if the player doesn't move - just use `desired_velocity:
    // Vec3::ZERO`. `TnuaController` starts without a basis, which will make the character collider
    // just fall.
    let yaw = transform.rotation.to_euler(EulerRot::YXZ).0;
    let yaw_quat = Quat::from_axis_angle(Vec3::Y, yaw);
    controller.basis(TnuaBuiltinWalk {
        // The `desired_velocity` determines how the character will move.
        desired_velocity: yaw_quat * last_move,
        // The `float_height` must be greater (even if by little) from the distance between the
        // character's center and the lowest point of its collider.
        float_height: PLAYER_FLOAT_HEIGHT,
        // Restrict the max slope so that the player cannot walk up slightly angled chairs.
        max_slope: TAU / 8.0,
        ..default()
    });
}

#[cfg_attr(feature = "hot_patch", hot)]
fn jump(trigger: Trigger<Fired<Jump>>, mut controllers: Query<&mut TnuaController>) {
    let mut controller = controllers.get_mut(trigger.target()).unwrap();
    controller.action(TnuaBuiltinJump {
        // The height is the only mandatory field of the jump button.
        height: 1.5,
        // `TnuaBuiltinJump` also has customization fields with sensible defaults.
        ..default()
    });
}
