use crate::{file_system_interaction::asset_loading::AudioAssets, GameState};
use bevy::prelude::*;
use bevy_kira_audio::prelude::{Audio, *};

/// Handles initialization of all sounds.
pub(super) fn plugin(app: &mut App) {
    app.add_plugins(AudioPlugin)
        .add_systems(OnExit(GameState::Loading), init_audio);
}

#[derive(Debug, Clone, Resource)]
pub(crate) struct AudioHandles {
    pub(crate) walking: Handle<AudioInstance>,
}

fn init_audio(mut commands: Commands, audio_assets: Res<AudioAssets>, audio: Res<Audio>) {
    audio.pause();
    let handle = audio
        .play(audio_assets.walking.clone())
        .looped()
        .with_volume(0.8)
        .handle();
    commands.insert_resource(AudioHandles { walking: handle });
}
