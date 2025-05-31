use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy_enhanced_input::events::Fired;

use super::default_input::Move;

use super::{Player, camera::PlayerCamera};

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub(crate) struct Noclip;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        toggle_noclip.run_if(input_just_pressed(KeyCode::KeyN)),
    );
    app.add_observer(move_camera_in_noclip);
}

fn toggle_noclip(player: Single<(Entity, Has<Noclip>), With<Player>>, mut commands: Commands) {
    let (entity, has_noclip) = player.into_inner();
    if has_noclip {
        commands.entity(entity).remove::<Noclip>();
    } else {
        commands.entity(entity).insert(Noclip);
    }
}

pub(crate) fn is_noclipping(player: Query<(), With<Noclip>>) -> bool {
    !player.is_empty()
}

fn move_camera_in_noclip(
    trigger: Trigger<Fired<Move>>,
    mut player_camera_parent: Single<&mut Transform, With<PlayerCamera>>,
    has_noclip: Single<Has<Noclip>, With<Player>>,
) {
    if !has_noclip.into_inner() {
        return;
    }

    let noclip_speed_multiplier = 0.25;
    let rotation = player_camera_parent.rotation;
    player_camera_parent.translation += rotation * noclip_speed_multiplier * trigger.value;
}
