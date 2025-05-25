//! Player pickup sound effects.

use avian_pickup::output::PropThrown;
use bevy::{audio::Volume, prelude::*};
#[cfg(feature = "hot_patch")]
use bevy_simple_subsecond_system::hot;

use crate::{
    PostPhysicsAppSystems, audio::SoundEffect, gameplay::player::assets::PlayerAssets,
    screens::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        play_throw_sound
            .run_if(in_state(Screen::Gameplay).and(on_event::<PropThrown>))
            .in_set(PostPhysicsAppSystems::PlaySounds),
    );
}

#[cfg_attr(feature = "hot_patch", hot)]
fn play_throw_sound(mut commands: Commands, player_assets: Res<PlayerAssets>) {
    let sound = player_assets.throw_sound.clone();

    commands.spawn((
        AudioPlayer(sound),
        PlaybackSettings::DESPAWN.with_volume(Volume::Linear(3.0)),
        SoundEffect,
    ));
}
