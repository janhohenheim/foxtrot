//! The title screen that appears when the game starts.

use bevy::prelude::*;

use crate::{asset_tracking::ResourceHandles, screens::Screen, theme::prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Title), spawn_title_screen);
}

fn spawn_title_screen(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Title Screen"),
        StateScoped(Screen::Title),
        #[cfg(feature = "native")]
        children![
            widget::button("Play", enter_loading_or_spawn_screen),
            widget::button("Settings", enter_settings_screen),
            widget::button("Credits", enter_credits_screen),
            widget::button("Exit", exit_app),
        ],
        #[cfg(not(feature = "native"))]
        children![
            widget::button("Play", enter_loading_or_spawn_screen),
            widget::button("Settings", enter_settings_screen),
            widget::button("Credits", enter_credits_screen),
        ],
    ));
}

fn enter_loading_or_spawn_screen(
    _: Trigger<Pointer<Click>>,
    resource_handles: Res<ResourceHandles>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    if resource_handles.is_all_done() {
        next_screen.set(Screen::SpawnLevel);
    } else {
        next_screen.set(Screen::Loading);
    }
}

fn enter_settings_screen(_: Trigger<Pointer<Click>>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Settings);
}

fn enter_credits_screen(_: Trigger<Pointer<Click>>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Credits);
}
#[cfg(feature = "native")]
fn exit_app(_: Trigger<Pointer<Click>>, mut app_exit: EventWriter<AppExit>) {
    app_exit.write(AppExit::Success);
}
