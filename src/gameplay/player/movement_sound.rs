use std::time::Duration;

use avian3d::prelude::LinearVelocity;
use bevy::prelude::*;
#[cfg(feature = "hot_patch")]
use bevy_simple_subsecond_system::hot;
use bevy_tnua::{builtins::TnuaBuiltinJumpState, prelude::*};

use crate::{PostPhysicsAppSystems, audio::sound_effect, screens::Screen};

use super::{Player, assets::PlayerAssets};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (play_jump_grunt, play_step_sound, play_land_sound)
            .run_if(in_state(Screen::Gameplay))
            .in_set(PostPhysicsAppSystems::PlaySounds),
    );
}

#[cfg_attr(feature = "hot_patch", hot)]
fn play_jump_grunt(
    mut commands: Commands,
    player: Single<&TnuaController, With<Player>>,
    mut player_assets: ResMut<PlayerAssets>,
    mut is_jumping: Local<bool>,
    mut sound_cooldown: Local<Option<Timer>>,
    time: Res<Time>,
) {
    let sound_cooldown = sound_cooldown
        .get_or_insert_with(|| Timer::new(Duration::from_millis(1000), TimerMode::Once));
    sound_cooldown.tick(time.delta());

    if player
        .concrete_action::<TnuaBuiltinJump>()
        .is_none_or(|x| matches!(x, (_, TnuaBuiltinJumpState::FallSection)))
    {
        *is_jumping = false;
        return;
    }
    if *is_jumping {
        return;
    }
    *is_jumping = true;

    if sound_cooldown.finished() {
        let rng = &mut rand::thread_rng();
        let grunt = player_assets.jump_grunts.pick(rng).clone();
        let jump_start = player_assets.jump_start_sounds.pick(rng).clone();

        commands.spawn(sound_effect(grunt));
        commands.spawn(sound_effect(jump_start));
        sound_cooldown.reset();
    }
}

#[cfg_attr(feature = "hot_patch", hot)]
fn play_step_sound(
    mut commands: Commands,
    player: Single<(&TnuaController, &LinearVelocity), With<Player>>,
    mut player_assets: ResMut<PlayerAssets>,
    time: Res<Time>,
    mut timer: Local<Option<Timer>>,
) {
    let timer =
        timer.get_or_insert_with(|| Timer::new(Duration::from_millis(300), TimerMode::Repeating));
    timer.tick(time.delta());
    if !timer.finished() {
        return;
    }

    let (controller, linear_velocity) = player.into_inner();
    if controller.is_airborne().unwrap_or(true) {
        return;
    }
    if linear_velocity.length_squared() < 5.0 {
        return;
    }
    let rng = &mut rand::thread_rng();
    let sound = player_assets.steps.pick(rng).clone();
    commands.spawn(sound_effect(sound));
}

#[cfg_attr(feature = "hot_patch", hot)]
fn play_land_sound(
    mut commands: Commands,
    player: Single<&TnuaController, With<Player>>,
    mut player_assets: ResMut<PlayerAssets>,
    mut was_airborne: Local<bool>,
) {
    let is_airborne = player.is_airborne().unwrap_or(true);
    if is_airborne {
        *was_airborne = true;
        return;
    }
    if !*was_airborne {
        return;
    }
    *was_airborne = false;

    let rng = &mut rand::thread_rng();
    let sound = player_assets.land_sounds.pick(rng).clone();
    commands.spawn(sound_effect(sound));
}
