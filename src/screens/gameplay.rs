//! The screen state for the main gameplay.

use bevy::prelude::*;

use crate::{asset_tracking::LoadResource, audio::Music, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<GameplayMusic>();
    app.add_systems(OnEnter(GameplayState::Playing), play_gameplay_music);
    app.add_systems(OnExit(GameplayState::Playing), stop_music);

    app.enable_state_scoped_entities::<GameplayState>();
}

#[derive(SubStates, Debug, Hash, PartialEq, Eq, Clone, Default)]
#[source(Screen = Screen::Gameplay)]
pub enum GameplayState {
    #[default]
    SpawningLevel,
    Playing,
}

#[derive(Resource, Asset, Reflect, Clone)]
pub struct GameplayMusic {
    #[dependency]
    handle: Handle<AudioSource>,
    entity: Option<Entity>,
}

impl FromWorld for GameplayMusic {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            handle: assets.load("audio/music/Fluffing A Duck.ogg"),
            entity: None,
        }
    }
}

fn play_gameplay_music(mut commands: Commands, mut music: ResMut<GameplayMusic>) {
    music.entity = Some(
        commands
            .spawn((
                AudioBundle {
                    source: music.handle.clone(),
                    settings: PlaybackSettings::LOOP,
                },
                Music,
            ))
            .id(),
    );
}

fn stop_music(mut commands: Commands, mut music: ResMut<GameplayMusic>) {
    if let Some(entity) = music.entity.take() {
        commands.entity(entity).despawn_recursive();
    }
}
