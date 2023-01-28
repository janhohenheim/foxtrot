use crate::file_system_interaction::asset_loading::AudioAssets;
use crate::movement::general_movement::Grounded;
use crate::player_control::actions::set_actions;
use crate::player_control::player_embodiment::Player;
use crate::util::trait_extension::Vec3Ext;
use crate::GameState;
use bevy::prelude::*;
use bevy_kira_audio::prelude::{Audio, *};
use bevy_rapier3d::control::KinematicCharacterControllerOutput;

pub struct InternalAudioPlugin;

// This plugin is responsible to control the game audio
impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AudioPlugin)
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(start_audio))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(control_flying_sound.after(set_actions)),
            );
    }
}

#[derive(Resource)]
struct WalkingAudio(Handle<AudioInstance>);

fn start_audio(mut commands: Commands, audio_assets: Res<AudioAssets>, audio: Res<Audio>) {
    audio.pause();
    let handle = audio
        .play(audio_assets.walking.clone())
        .looped()
        .with_volume(0.8)
        .handle();
    commands.insert_resource(WalkingAudio(handle));
}

fn control_flying_sound(
    character_query: Query<(&KinematicCharacterControllerOutput, &Grounded), With<Player>>,
    audio: Res<WalkingAudio>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
) {
    for (output, grounded) in character_query.iter() {
        if let Some(instance) = audio_instances.get_mut(&audio.0) {
            let is_in_air = grounded.time_since_last_grounded.is_active();
            let has_horizontal_movement = !output.effective_translation.x0z().is_approx_zero();
            let is_moving_on_ground = has_horizontal_movement && !is_in_air;
            if is_moving_on_ground {
                instance.resume(AudioTween::default());
            } else {
                instance.pause(AudioTween::default());
            }
        }
    }
}
