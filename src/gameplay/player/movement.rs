use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;
use bevy_tnua::prelude::*;

use super::input::{Jump, Move};

use super::PLAYER_FLOAT_HEIGHT;
use super::{Player, camera::PlayerCameraParent};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(PreUpdate, reset_movement);
    app.add_observer(apply_movement).add_observer(jump);
}

fn reset_movement(mut controllers: Query<&mut TnuaController, With<Player>>) {
    for mut controller in controllers.iter_mut() {
        controller.basis(TnuaBuiltinWalk {
            float_height: PLAYER_FLOAT_HEIGHT,
            ..Default::default()
        });
    }
}

fn apply_movement(
    trigger: Trigger<Fired<Move>>,
    mut controllers: Query<&mut TnuaController, With<Player>>,
    transform: Option<Single<&Transform, With<PlayerCameraParent>>>,
) {
    let Ok(mut controller) = controllers.get_mut(trigger.entity()) else {
        error!("Triggered movement for entity with missing components");
        return;
    };
    let Some(transform) = transform else {
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
        // `TnuaBuiltinWalk` has many other fields for customizing the movement - but they have
        // sensible defaults. Refer to the `TnuaBuiltinWalk`'s documentation to learn what they do.
        spring_strength: 800.0,
        ..Default::default()
    });
}

fn jump(trigger: Trigger<Fired<Jump>>, mut controllers: Query<&mut TnuaController>) {
    let mut controller = controllers.get_mut(trigger.entity()).unwrap();
    controller.action(TnuaBuiltinJump {
        // The height is the only mandatory field of the jump button.
        height: 1.5,
        // `TnuaBuiltinJump` also has customization fields with sensible defaults.
        ..Default::default()
    });
}
