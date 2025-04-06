use bevy::prelude::*;

/// An organizational marker component that should be added to a spawned [`AudioPlayer`] if it is in the
/// general "music" category (ex: global background music, soundtrack, etc).
///
/// This can then be used to query for and operate on sounds in that category. For example:
///
/// ```
/// use bevy::{audio::Volume, prelude::*};
/// use foxtrot::audio::Music;
///
/// fn set_music_volume(mut sink_query: Query<&mut AudioSink, With<Music>>) {
///     for mut sink in &mut sink_query {
///         sink.set_volume(Volume::Linear(0.5));
///     }
/// }
/// ```
#[derive(Component, Default)]
pub(crate) struct Music;

/// An organizational marker component that should be added to a spawned [`AudioPlayer`] if it is in the
/// general "sound effect" category (ex: footsteps, the sound of a magic spell, a door opening).
///
/// This can then be used to query for and operate on sounds in that category. For example:
///
/// ```
/// use bevy::{audio::Volume, prelude::*};
/// use foxtrot::audio::SoundEffect;
///
/// fn set_sound_effect_volume(mut sink_query: Query<&mut AudioSink, With<SoundEffect>>) {
///     for mut sink in &mut sink_query {
///         sink.set_volume(Volume::Linear(0.5));
///     }
/// }
/// ```
#[derive(Component, Default)]
pub(crate) struct SoundEffect;
