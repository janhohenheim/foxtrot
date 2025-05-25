//! The title screen that appears when the game starts.

use bevy::{prelude::*, window::CursorGrabMode};
#[cfg(feature = "hot_patch")]
use bevy_simple_subsecond_system::hot;

use crate::{screens::Screen, theme::prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Title), spawn_title_screen);
}

#[cfg_attr(feature = "hot_patch", hot)]
fn spawn_title_screen(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Title Screen"),
        StateScoped(Screen::Title),
        #[cfg(feature = "native")]
        children![
            widget::button("Play", enter_loading_screen),
            widget::button("Settings", enter_settings_screen),
            widget::button("Credits", enter_credits_screen),
            widget::button("Exit", exit_app),
        ],
        #[cfg(not(feature = "native"))]
        children![
            widget::button("Play", enter_loading_screen),
            widget::button("Settings", enter_settings_screen),
            widget::button("Credits", enter_credits_screen),
        ],
    ));
}

#[cfg_attr(feature = "hot_patch", hot)]
fn enter_loading_screen(
    _trigger: Trigger<Pointer<Click>>,
    mut next_screen: ResMut<NextState<Screen>>,
    mut window: Single<&mut Window>,
) {
    next_screen.set(Screen::Loading);
    window.cursor_options.grab_mode = CursorGrabMode::Locked;
}

#[cfg_attr(feature = "hot_patch", hot)]
fn enter_settings_screen(
    _trigger: Trigger<Pointer<Click>>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    next_screen.set(Screen::Settings);
}

#[cfg_attr(feature = "hot_patch", hot)]
fn enter_credits_screen(
    _trigger: Trigger<Pointer<Click>>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    next_screen.set(Screen::Credits);
}
#[cfg(feature = "native")]
fn exit_app(_trigger: Trigger<Pointer<Click>>, mut app_exit: EventWriter<AppExit>) {
    app_exit.write(AppExit::Success);
}
