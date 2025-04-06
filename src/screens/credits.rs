//! A credits screen that can be accessed from the title screen.

use bevy::prelude::*;

use crate::{asset_tracking::LoadResource, audio::Music, screens::Screen, theme::prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Credits), spawn_credits_screen);

    app.register_type::<CreditsMusic>();
    app.load_resource::<CreditsMusic>();
    app.add_systems(OnEnter(Screen::Credits), start_credits_music);
    app.add_systems(OnExit(Screen::Credits), stop_credits_music);
}

fn spawn_credits_screen(mut commands: Commands) {
    commands
        .ui_root()
        .insert(StateScoped(Screen::Credits))
        .with_children(|parent| {
            parent.header("Made by");
            parent.label("Joe Shmoe - Implemented aligator wrestling AI");
            parent.label("Jane Doe - Made the music for the alien invasion");

            parent.header("Assets");
            parent.label("Bevy logo - All rights reserved by the Bevy Foundation. Permission granted for splash screen use when unmodified.");
            parent.label("Ducky sprite - CC0 by Caz Creates Games");
            parent.label("Button SFX - CC0 by Jaszunio15");
            parent.label("Music - CC BY 3.0 by Kevin MacLeod");

            parent.button("Back").observe(enter_title_screen);
        });
}

fn enter_title_screen(_: Trigger<Pointer<Pressed>>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
struct CreditsMusic {
    #[dependency]
    music: Handle<AudioSource>,
    entity: Option<Entity>,
}

impl FromWorld for CreditsMusic {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            music: assets.load("audio/music/Monkeys Spinning Monkeys.ogg"),
            entity: None,
        }
    }
}

fn start_credits_music(mut commands: Commands, mut music: ResMut<CreditsMusic>) {
    music.entity = Some(
        commands
            .spawn((
                AudioPlayer(music.music.clone()),
                PlaybackSettings::LOOP,
                Music,
            ))
            .id(),
    );
}

fn stop_credits_music(mut commands: Commands, mut music: ResMut<CreditsMusic>) {
    if let Some(entity) = music.entity.take() {
        commands.entity(entity).despawn();
    }
}
