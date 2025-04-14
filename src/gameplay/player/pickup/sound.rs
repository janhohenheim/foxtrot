use avian_pickup::output::PropThrown;
use bevy::{audio::Volume, prelude::*};

use crate::{audio::SoundEffect, gameplay::player::assets::PlayerAssets, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        play_throw_sound
            .run_if(on_event::<PropThrown>)
            .run_if(in_state(Screen::Gameplay)),
    );
}

fn play_throw_sound(mut commands: Commands, player_assets: Res<PlayerAssets>) {
    let sound = player_assets.throw_sound.clone();

    commands.spawn((
        AudioPlayer(sound.clone()),
        PlaybackSettings::DESPAWN.with_volume(Volume::new(3.0)),
        SoundEffect,
    ));
}
