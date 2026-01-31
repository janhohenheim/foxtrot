//! NPC sound handling. The only sound is a step sound that plays when the NPC is walking.

use super::{Npc, assets::NpcAssets};
use crate::{PostPhysicsAppSystems, audio::SpatialPool, screens::Screen};
use avian3d::prelude::LinearVelocity;
use bevy::prelude::*;
use bevy_ahoy::CharacterControllerState;
use bevy_seedling::prelude::*;

use std::time::Duration;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        play_step_sound
            .run_if(in_state(Screen::Gameplay))
            .in_set(PostPhysicsAppSystems::PlaySounds),
    );
}

fn play_step_sound(
    mut commands: Commands,
    npc: Single<(Entity, &CharacterControllerState, &LinearVelocity), With<Npc>>,
    mut npc_assets: ResMut<NpcAssets>,
    time: Res<Time>,
    mut timer: Local<Option<Timer>>,
) {
    let base_millis = 300;
    let timer = timer.get_or_insert_with(|| {
        Timer::new(Duration::from_millis(base_millis), TimerMode::Repeating)
    });
    timer.tick(time.delta());
    if !timer.is_finished() {
        return;
    }

    let (entity, state, linear_velocity) = npc.into_inner();
    if state.grounded.is_none() {
        return;
    }
    let speed = linear_velocity.length();
    if speed < 1.0 {
        return;
    }
    // At speed = 5 m/s, halve the duration.
    let speed_to_half_duration = 5.0;
    let factor = 1.0 - (speed - speed_to_half_duration) / speed_to_half_duration;
    timer.set_duration(Duration::from_millis((base_millis as f32 * factor) as u64));
    let rng = &mut rand::rng();
    let sound_effect = npc_assets.steps.pick(rng).clone();

    commands.entity(entity).with_child((
        Transform::default(),
        SamplePlayer::new(sound_effect).with_volume(Volume::Linear(1.6)),
        PlaybackSettings {
            speed: 1.5,
            ..default()
        },
        SpatialPool,
    ));
}
