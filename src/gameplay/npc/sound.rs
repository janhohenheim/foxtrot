//! NPC sound handling. The only sound is a step sound that plays when the NPC is walking.

use super::{Npc, assets::NpcAssets};
use crate::{PostPhysicsAppSystems, audio::SoundEffect, screens::Screen};
use avian3d::prelude::LinearVelocity;
use bevy::{
    audio::{SpatialScale, Volume},
    prelude::*,
};
#[cfg(feature = "hot_patch")]
use bevy_simple_subsecond_system::hot;
use bevy_tnua::prelude::*;
use std::time::Duration;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        play_step_sound
            .run_if(in_state(Screen::Gameplay))
            .in_set(PostPhysicsAppSystems::PlaySounds),
    );
}

#[cfg_attr(feature = "hot_patch", hot)]
fn play_step_sound(
    mut commands: Commands,
    npc: Single<(Entity, &TnuaController, &LinearVelocity), With<Npc>>,
    mut npc_assets: ResMut<NpcAssets>,
    time: Res<Time>,
    mut timer: Local<Option<Timer>>,
) {
    let base_millis = 300;
    let timer = timer.get_or_insert_with(|| {
        Timer::new(Duration::from_millis(base_millis), TimerMode::Repeating)
    });
    timer.tick(time.delta());
    if !timer.finished() {
        return;
    }

    let (entity, controller, linear_velocity) = npc.into_inner();
    if controller.is_airborne().unwrap_or(true) {
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
    let rng = &mut rand::thread_rng();
    let sound_effect = npc_assets.steps.pick(rng).clone();

    commands.entity(entity).with_child((
        Transform::default(),
        AudioPlayer(sound_effect),
        PlaybackSettings::DESPAWN
            .with_spatial(true)
            .with_speed(1.5)
            .with_volume(Volume::Linear(1.6))
            .with_spatial_scale(SpatialScale::new(1.0 / 3.6)),
        SoundEffect,
    ));
}
