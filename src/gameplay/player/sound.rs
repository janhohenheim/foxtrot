use bevy::prelude::*;
use bevy_tnua::{builtins::TnuaBuiltinJumpState, prelude::*};
use rand::seq::SliceRandom as _;

use crate::{audio::SoundEffect, screens::Screen};

use super::{Player, assets::PlayerAssets};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, play_jump_grunt.run_if(in_state(Screen::Gameplay)));
}

fn play_jump_grunt(
    mut commands: Commands,
    player: Single<&TnuaController, With<Player>>,
    player_assets: Res<PlayerAssets>,
    mut is_jumping: Local<bool>,
) {
    let Some((_jump, jump_state)) = player.concrete_action::<TnuaBuiltinJump>() else {
        return;
    };
    let started_jumping = matches!(
        jump_state,
        TnuaBuiltinJumpState::StartingJump { .. }
            | TnuaBuiltinJumpState::SlowDownTooFastSlopeJump { .. }
    );
    if !started_jumping {
        *is_jumping = false;
        return;
    }
    if *is_jumping {
        return;
    }
    *is_jumping = true;

    let rng = &mut rand::thread_rng();
    let sound_effect = player_assets.jump_grunts.choose(rng).unwrap();

    commands.spawn((
        AudioPlayer(sound_effect.clone()),
        PlaybackSettings::DESPAWN,
        SoundEffect,
    ));
}
