//! The screen state for the main gameplay.

use bevy::{audio::Volume, input::common_conditions::input_just_pressed, prelude::*};

use crate::{AppSet, asset_tracking::LoadResource, audio::Music, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<GameplayMusic>();
    app.load_resource::<GameplayMusic>();
    app.add_systems(OnEnter(Screen::SpawnLevel), start_gameplay_music);
    app.add_systems(OnExit(Screen::Gameplay), stop_gameplay_music);

    app.add_systems(
        Update,
        return_to_title_screen
            .run_if(in_state(Screen::Gameplay).and(input_just_pressed(KeyCode::Escape)))
            .in_set(AppSet::Update),
    );
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
struct GameplayMusic {
    #[dependency]
    music: Handle<AudioSource>,
    entity: Option<Entity>,
}

impl FromWorld for GameplayMusic {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            music: assets.load("audio/music/Ambiance_Rain_Calm_Loop_Stereo.ogg"),
            entity: None,
        }
    }
}

fn start_gameplay_music(mut commands: Commands, mut music: ResMut<GameplayMusic>) {
    music.entity = Some(
        commands
            .spawn((
                AudioPlayer(music.music.clone()),
                PlaybackSettings::LOOP.with_volume(Volume::new(1.5)),
                Music,
            ))
            .id(),
    );
}

fn stop_gameplay_music(mut commands: Commands, mut music: ResMut<GameplayMusic>) {
    if let Some(entity) = music.entity.take() {
        commands.entity(entity).despawn();
    }
}

fn return_to_title_screen(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}
