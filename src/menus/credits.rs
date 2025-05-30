//! A credits menu.

use bevy::{
    ecs::spawn::SpawnIter, input::common_conditions::input_just_pressed, prelude::*, ui::Val::*,
};
#[cfg(feature = "hot_patch")]
use bevy_simple_subsecond_system::hot;

use crate::{
    Pause,
    asset_tracking::LoadResource,
    audio::music,
    menus::Menu,
    theme::{palette::SCREEN_BACKGROUND, prelude::*},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Credits), spawn_credits_menu);
    app.add_systems(
        Update,
        go_back.run_if(in_state(Menu::Credits).and(input_just_pressed(KeyCode::Escape))),
    );

    app.register_type::<CreditsAssets>();
    app.load_resource::<CreditsAssets>();
    app.add_systems(OnEnter(Menu::Credits), start_credits_music);
}

#[cfg_attr(feature = "hot_patch", hot)]
fn spawn_credits_menu(mut commands: Commands, paused: Res<State<Pause>>) {
    let mut entity_commands = commands.spawn((
        widget::ui_root("Credits Screen"),
        StateScoped(Menu::Credits),
        GlobalZIndex(2),
        children![
            widget::header("Created by"),
            created_by(),
            widget::header("Assets"),
            assets(),
            widget::button("Back", go_back_on_click),
        ],
    ));
    if paused.get() == &Pause(false) {
        entity_commands.insert(BackgroundColor(SCREEN_BACKGROUND));
    }
}

fn created_by() -> impl Bundle {
    grid(vec![
        ["Joe Shmoe", "Implemented alligator wrestling AI"],
        ["Jane Doe", "Made the music for the alien invasion"],
    ])
}

fn assets() -> impl Bundle {
    grid(vec![
        [
            "Bevy logo",
            "All rights reserved by the Bevy Foundation, permission granted for splash screen use when unmodified",
        ],
        ["Button SFX", "CC0 by Jaszunio15"],
        ["Music", "CC BY 3.0 by Kevin MacLeod"],
        ["Ambient music and Footstep SFX", "CC0 by NOX SOUND"],
        [
            "Throw SFX",
            "FilmCow Royalty Free SFX Library License Agreement by Jason Steele",
        ],
        [
            "Fox model",
            "CC0 1.0 Universal by PixelMannen (model), CC BY 4.0 International by tomkranis (Rigging & Animation), CC BY 4.0 International by AsoboStudio and scurest (Conversion to glTF)",
        ],
        [
            "Player model",
            "You can use it commercially without the need to credit me by Drillimpact",
        ],
        ["Vocals", "CC BY 4.0 by Dillon Becker"],
        ["Night Sky HDRI 001", "CC0 by ambientCG"],
        [
            "Rest of the assets",
            "CC BY-NC-SA 3.0 by The Dark Mod Team, converted to Bevy-friendly assets by Jan Hohenheim",
        ],
    ])
}

fn grid(content: Vec<[&'static str; 2]>) -> impl Bundle {
    (
        Name::new("Grid"),
        Node {
            display: Display::Grid,
            row_gap: Px(10.0),
            column_gap: Px(30.0),
            grid_template_columns: RepeatedGridTrack::px(2, 400.0),
            ..default()
        },
        Children::spawn(SpawnIter(content.into_iter().flatten().enumerate().map(
            |(i, text)| {
                (
                    widget::label_small(text),
                    Node {
                        justify_self: if i % 2 == 0 {
                            JustifySelf::End
                        } else {
                            JustifySelf::Start
                        },
                        ..default()
                    },
                )
            },
        ))),
    )
}

fn go_back_on_click(_: Trigger<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Main);
}

fn go_back(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Main);
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
struct CreditsAssets {
    #[dependency]
    music: Handle<AudioSource>,
}

impl FromWorld for CreditsAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            music: assets.load("audio/music/Monkeys Spinning Monkeys.ogg"),
        }
    }
}

#[cfg_attr(feature = "hot_patch", hot)]
fn start_credits_music(mut commands: Commands, credits_music: Res<CreditsAssets>) {
    commands.spawn((
        Name::new("Credits Music"),
        StateScoped(Menu::Credits),
        music(credits_music.music.clone()),
    ));
}
