use bevy::prelude::*;
use bevy_enhanced_input::events::{Fired, Started};

use super::debug_input::Noclip as NoclipInput;
use super::default_input::Move;

use super::camera::PlayerCamera;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub(crate) struct Noclip;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(toggle_noclip);
    app.add_observer(move_camera_in_noclip);
}

fn toggle_noclip(
    trigger: Trigger<Started<NoclipInput>>,
    noclipping: Query<(), With<Noclip>>,
    mut commands: Commands,
) {
    let entity = trigger.target();
    let is_noclipping = noclipping.contains(entity);
    if is_noclipping {
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
    noclipping: Query<(), With<Noclip>>,
) {
    if !noclipping.contains(trigger.target()) {
        return;
    }

    let noclip_speed_multiplier = 0.25;
    let rotation = player_camera_parent.rotation;
    player_camera_parent.translation += rotation * noclip_speed_multiplier * trigger.value;
}
