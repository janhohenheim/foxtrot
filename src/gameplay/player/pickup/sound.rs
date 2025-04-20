//! Player pickup sound effects.

use avian_pickup::output::PropThrown;
use bevy::{audio::Volume, prelude::*};

use crate::{AppSet, audio::SoundEffect, gameplay::player::assets::PlayerAssets, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        play_throw_sound
            .run_if(in_state(Screen::Gameplay).and(on_event::<PropThrown>))
            .in_set(AppSet::PlaySounds),
    );
}

fn play_throw_sound(mut commands: Commands, player_assets: Res<PlayerAssets>) {
    let sound = player_assets.throw_sound.clone();

    commands.spawn((
        AudioPlayer(sound),
        PlaybackSettings::DESPAWN.with_volume(Volume::new(3.0)),
        SoundEffect,
    ));
}
