use std::f32::consts::TAU;

use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;
use bevy_tnua::prelude::*;

use super::default_input::{Jump, Move};

use super::PLAYER_FLOAT_HEIGHT;
use super::{Player, camera::PlayerCamera};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(PreUpdate, reset_movement);
    app.add_observer(apply_movement);
    app.add_observer(jump);
}

fn reset_movement(mut controllers: Query<&mut TnuaController, With<Player>>) {
    for mut controller in &mut controllers {
        controller.basis(TnuaBuiltinWalk {
            float_height: PLAYER_FLOAT_HEIGHT,
            ..default()
        });
    }
}

fn apply_movement(
    trigger: Trigger<Fired<Move>>,
    mut controllers: Query<&mut TnuaController, With<Player>>,
    transform: Single<&Transform, With<PlayerCamera>>,
) {
    let Ok(mut controller) = controllers.get_mut(trigger.target()) else {
        error!("Triggered movement for entity with missing components");
        return;
    };
    // Feed the basis every frame. Even if the player doesn't move - just use `desired_velocity:
    // Vec3::ZERO`. `TnuaController` starts without a basis, which will make the character collider
    // just fall.
    let yaw = transform.rotation.to_euler(EulerRot::YXZ).0;
    let yaw_quat = Quat::from_axis_angle(Vec3::Y, yaw);
    controller.basis(TnuaBuiltinWalk {
        // The `desired_velocity` determines how the character will move.
        desired_velocity: yaw_quat * trigger.value,
        // The `float_height` must be greater (even if by little) from the distance between the
        // character's center and the lowest point of its collider.
        float_height: PLAYER_FLOAT_HEIGHT,
        // Restrict the max slope so that the player cannot walk up slightly angled chairs.
        max_slope: TAU / 8.0,
        ..default()
    });
}

fn jump(trigger: Trigger<Fired<Jump>>, mut controllers: Query<&mut TnuaController>) {
    let mut controller = controllers.get_mut(trigger.target()).unwrap();
    controller.action(TnuaBuiltinJump {
        // The height is the only mandatory field of the jump button.
        height: 1.5,
        // `TnuaBuiltinJump` also has customization fields with sensible defaults.
        ..default()
    });
}
