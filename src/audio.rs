use bevy::{audio::Volume, prelude::*};

/// An organizational marker component that should be added to a spawned [`AudioPlayer`] if it's in the
/// general "music" category (e.g. global background music, soundtrack).
///
/// This can then be used to query for and operate on sounds in that category.
#[derive(Component, Default)]
pub(crate) struct Music;

/// A music audio instance.
pub(crate) fn music(handle: Handle<AudioSource>) -> impl Bundle {
    (AudioPlayer(handle), PlaybackSettings::LOOP, Music)
}

/// An organizational marker component that should be added to a spawned [`AudioPlayer`] if it's in the
/// general "sound effect" category (e.g. footsteps, the sound of a magic spell, a door opening).
///
/// This can then be used to query for and operate on sounds in that category.
#[derive(Component, Default)]
pub(crate) struct SoundEffect;

/// A sound effect audio instance.
pub(crate) fn sound_effect(handle: Handle<AudioSource>) -> impl Bundle {
    (AudioPlayer(handle), PlaybackSettings::DESPAWN, SoundEffect)
}

pub(crate) const DEFAULT_VOLUME: Volume = Volume::Linear(0.3);

pub(crate) fn max_volume() -> Volume {
    DEFAULT_VOLUME + Volume::Decibels(5.0)
}
