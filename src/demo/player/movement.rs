use std::f32::consts::FRAC_PI_2;

use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*};
use bevy_enhanced_input::prelude::*;
use bevy_tnua::prelude::*;

use super::{Player, camera::CameraSensitivity};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(PreUpdate, reset_movement);
    app.add_observer(apply_movement).add_observer(jump);
    app.add_systems(Update, rotate_player);
}

// All actions should implement the `InputAction` trait.
// It can be done manually, but we provide a derive for convenience.
// The only necessary parameter is `output`, which defines the output type.
#[derive(Debug, InputAction)]
#[input_action(output = Vec3)]
pub(crate) struct Move;

#[derive(Debug, InputAction)]
#[input_action(output = bool)]
pub(crate) struct Jump;

fn reset_movement(mut controllers: Query<&mut TnuaController, With<Player>>) {
    for mut controller in controllers.iter_mut() {
        controller.basis(TnuaBuiltinWalk {
            float_height: 1.1,
            ..Default::default()
        });
    }
}

fn apply_movement(
    trigger: Trigger<Fired<Move>>,
    mut controllers: Query<(&Transform, &mut TnuaController), With<Player>>,
) {
    let Ok((transform, mut controller)) = controllers.get_mut(trigger.entity()) else {
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
        float_height: 1.1,
        // `TnuaBuiltinWalk` has many other fields for customizing the movement - but they have
        // sensible defaults. Refer to the `TnuaBuiltinWalk`'s documentation to learn what they do.
        ..Default::default()
    });
}

fn jump(trigger: Trigger<Fired<Jump>>, mut controllers: Query<&mut TnuaController>) {
    let mut controller = controllers.get_mut(trigger.entity()).unwrap();
    controller.action(TnuaBuiltinJump {
        // The height is the only mandatory field of the jump button.
        height: 4.0,
        // `TnuaBuiltinJump` also has customization fields with sensible defaults.
        ..Default::default()
    });
}

fn rotate_player(
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    mut player: Query<(&mut Transform, &CameraSensitivity), With<Player>>,
) {
    let Ok((mut transform, camera_sensitivity)) = player.get_single_mut() else {
        return;
    };
    let delta = accumulated_mouse_motion.delta;

    if delta != Vec2::ZERO {
        // Note that we are not multiplying by delta_time here.
        // The reason is that for mouse movement, we already get the full movement that happened since the last frame.
        // This means that if we multiply by delta_time, we will get a smaller rotation than intended by the user.
        // This situation is reversed when reading e.g. analog input from a gamepad however, where the same rules
        // as for keyboard input apply. Such an input should be multiplied by delta_time to get the intended rotation
        // independent of the framerate.
        let delta_yaw = -delta.x * camera_sensitivity.x;
        let delta_pitch = -delta.y * camera_sensitivity.y;

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
