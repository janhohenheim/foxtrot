//! The pause menu.

use std::any::Any as _;

use crate::{gameplay::crosshair::CrosshairState, menus::Menu, screens::Screen, theme::widget};
use bevy::{input::common_conditions::input_just_pressed, prelude::*};
#[cfg(feature = "hot_patch")]
use bevy_simple_subsecond_system::hot;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Pause), spawn_pause_menu);
    app.add_systems(
        Update,
        go_back.run_if(in_state(Menu::Pause).and(input_just_pressed(KeyCode::Escape))),
    );
}

fn spawn_pause_menu(
    mut commands: Commands,
    mut crosshair: Single<&mut CrosshairState>,
    mut time: ResMut<Time<Virtual>>,
) {
    commands.spawn((
        widget::ui_root("Pause Menu"),
        GlobalZIndex(2),
        StateScoped(Menu::Pause),
        children![
            widget::header("Game paused"),
            widget::button("Continue", close_menu),
            widget::button("Settings", open_settings_menu),
            widget::button("Quit to title", quit_to_title),
        ],
    ));
    crosshair
        .wants_free_cursor
        .insert(spawn_pause_menu.type_id());
    time.pause();
}

#[cfg_attr(feature = "hot_patch", hot)]
fn open_settings_menu(_trigger: Trigger<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Settings);
}

#[cfg_attr(feature = "hot_patch", hot)]
fn close_menu(
    _trigger: Trigger<Pointer<Click>>,
    mut next_menu: ResMut<NextState<Menu>>,
    mut crosshair: Single<&mut CrosshairState>,
    mut time: ResMut<Time<Virtual>>,
) {
    next_menu.set(Menu::None);
    crosshair
        .wants_free_cursor
        .remove(&spawn_pause_menu.type_id());
    time.unpause();
}

#[cfg_attr(feature = "hot_patch", hot)]
fn quit_to_title(
    _trigger: Trigger<Pointer<Click>>,
    mut next_screen: ResMut<NextState<Screen>>,
    mut crosshair: Single<&mut CrosshairState>,
    mut time: ResMut<Time<Virtual>>,
) {
    next_screen.set(Screen::Title);
    crosshair
        .wants_free_cursor
        .remove(&spawn_pause_menu.type_id());
    time.unpause();
}

#[cfg_attr(feature = "hot_patch", hot)]
fn go_back(
    mut next_menu: ResMut<NextState<Menu>>,
    mut crosshair: Single<&mut CrosshairState>,
    mut time: ResMut<Time<Virtual>>,
) {
    next_menu.set(Menu::None);
    crosshair
        .wants_free_cursor
        .remove(&spawn_pause_menu.type_id());
    time.unpause();
}
