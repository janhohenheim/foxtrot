//! A loading screen during which game assets are loaded.
//! This reduces stuttering, especially for audio on WASM.

use bevy::prelude::*;
use bevy_yarnspinner::prelude::YarnProject;

use crate::{AppSet, asset_tracking::ResourceHandles, screens::Screen, theme::prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Loading), spawn_loading_screen);

    app.add_systems(
        Update,
        continue_to_title_screen
            .run_if(in_state(Screen::Loading).and(all_assets_loaded))
            .in_set(AppSet::Update),
    );
}

fn spawn_loading_screen(mut commands: Commands) {
    commands
        .ui_root()
        .insert(StateScoped(Screen::Loading))
        .with_children(|parent| {
            parent.label("Loading...").insert(Node {
                justify_content: JustifyContent::Center,
                ..default()
            });
        });
}

fn continue_to_title_screen(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::SpawnLevel);
}

fn all_assets_loaded(
    resource_handles: Res<ResourceHandles>,
    yarn_project: Option<Res<YarnProject>>,
) -> bool {
    resource_handles.is_all_done() && yarn_project.is_some()
}
