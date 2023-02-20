use crate::file_system_interaction::asset_loading::AudioAssets;
use crate::util::log_error::log_errors;
use crate::GameState;
use anyhow::{Context, Result};
use bevy::prelude::*;
use bevy_kira_audio::prelude::{Audio, *};

/// Handles initialization of all sounds.
pub struct InternalAudioPlugin;

// This plugin is responsible to control the game audio
impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AudioPlugin)
            .add_system_set(SystemSet::on_exit(GameState::Loading).with_system(init_audio))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(pause_audio_when_time_is_paused.pipe(log_errors)),
            );
    }
}

#[derive(Debug, Clone, Resource)]
pub struct AudioHandles {
    pub walking: Handle<AudioInstance>,
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

fn pause_audio_when_time_is_paused(
    time: Res<Time>,
    audio_handles: Res<AudioHandles>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
) -> Result<()> {
    if !time.is_paused() {
        return Ok(());
    }
    let handles = [&audio_handles.walking];
    for handle in handles {
        let audio_instance = audio_instances
            .get_mut(handle)
            .context("Failed to get audio instance from handle")?;
        audio_instance.pause(default());
    }
    Ok(())
}
