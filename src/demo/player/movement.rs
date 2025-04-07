use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;
use bevy_tnua::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(apply_movement).add_observer(jump);
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

fn apply_movement(
    trigger: Trigger<Fired<Move>>,
    mut controllers: Query<(&Transform, &mut TnuaController)>,
) {
    let Ok((transform, mut controller)) = controllers.get_mut(trigger.entity()) else {
        error!("Triggered movement for entity with missing components");
        return;
    };
    // Feed the basis every frame. Even if the player doesn't move - just use `desired_velocity:
    // Vec3::ZERO`. `TnuaController` starts without a basis, which will make the character collider
    // just fall.
    controller.basis(TnuaBuiltinWalk {
        // The `desired_velocity` determines how the character will move.
        desired_velocity: transform.rotation * trigger.value,
        // The `float_height` must be greater (even if by little) from the distance between the
        // character's center and the lowest point of its collider.
        float_height: 1.5,
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
