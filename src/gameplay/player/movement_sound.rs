use std::time::Duration;

use super::{Player, assets::PlayerAssets};
use crate::audio::SpatialPool;
use crate::{PostPhysicsAppSystems, screens::Screen};
use avian3d::prelude::LinearVelocity;
use bevy::prelude::*;
use bevy_ahoy::prelude::*;
use bevy_seedling::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (play_jump_grunt, play_step_sound, play_land_sound)
            .run_if(in_state(Screen::Gameplay))
            .in_set(PostPhysicsAppSystems::PlaySounds),
    );
}

fn play_jump_grunt(
    mut commands: Commands,
    player: Single<(Entity, &CharacterControllerState), With<Player>>,
    mut player_assets: ResMut<PlayerAssets>,
    mut is_jumping: Local<bool>,
    mut sound_cooldown: Local<Option<Timer>>,
    time: Res<Time>,
) {
    let sound_cooldown = sound_cooldown
        .get_or_insert_with(|| Timer::new(Duration::from_millis(1000), TimerMode::Once));
    sound_cooldown.tick(time.delta());

    let (entity, state) = player.into_inner();
    // TODO: use actual observer
    if state.grounded.is_some() {
        *is_jumping = false;
        return;
    }
    if *is_jumping {
        return;
    }
    *is_jumping = true;

    if sound_cooldown.is_finished() {
        let rng = &mut rand::rng();
        let grunt = player_assets.jump_grunts.pick(rng).clone();
        let jump_start = player_assets.jump_start_sounds.pick(rng).clone();

        commands.entity(entity).with_child((
            SamplePlayer::new(grunt),
            SpatialPool,
            Transform::default(),
        ));
        commands.entity(entity).with_child((
            SamplePlayer::new(jump_start),
            SpatialPool,
            Transform::default(),
        ));
        sound_cooldown.reset();
    }
}

fn play_step_sound(
    mut commands: Commands,
    player: Single<(Entity, &CharacterControllerState, &LinearVelocity), With<Player>>,
    mut player_assets: ResMut<PlayerAssets>,
    time: Res<Time>,
    mut timer: Local<Option<Timer>>,
) {
    let timer =
        timer.get_or_insert_with(|| Timer::new(Duration::from_millis(300), TimerMode::Repeating));
    timer.tick(time.delta());
    if !timer.is_finished() {
        return;
    }

    let (entity, state, linear_velocity) = player.into_inner();
    if state.grounded.is_none() {
        return;
    }
    if linear_velocity.length_squared() < 5.0 {
        return;
    }
    let rng = &mut rand::rng();
    let sound = player_assets.steps.pick(rng).clone();
    commands.entity(entity).with_child((
        SamplePlayer::new(sound),
        SpatialPool,
        Transform::default(),
    ));
}

fn play_land_sound(
    mut commands: Commands,
    player: Single<(Entity, &CharacterControllerState), With<Player>>,
    mut player_assets: ResMut<PlayerAssets>,
    mut was_airborne: Local<bool>,
) {
    let (entity, state) = player.into_inner();
    let is_airborne = state.grounded.is_none();
    if is_airborne {
        *was_airborne = true;
        return;
    }
    if !*was_airborne {
        return;
    }
    *was_airborne = false;

    let rng = &mut rand::rng();
    let sound = player_assets.land_sounds.pick(rng).clone();
    commands.entity(entity).with_child((
        SamplePlayer::new(sound),
        SpatialPool,
        Transform::default(),
    ));
}
