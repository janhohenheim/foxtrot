//! The pause menu.

use std::any::Any as _;

use crate::{
    gameplay::{crosshair::CrosshairState, player::input::BlocksInput},
    menus::Menu,
    screens::Screen,
    theme::widget,
};
use bevy::{input::common_conditions::input_just_pressed, prelude::*};

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
    mut blocks_input: ResMut<BlocksInput>,
) {
    commands.spawn((
        widget::ui_root("Pause Menu"),
        GlobalZIndex(2),
        DespawnOnExit(Menu::Pause),
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
    blocks_input.insert(spawn_pause_menu.type_id());
    time.pause();
}

fn open_settings_menu(_on: On<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Settings);
}

fn close_menu(
    _on: On<Pointer<Click>>,
    mut next_menu: ResMut<NextState<Menu>>,
    mut crosshair: Single<&mut CrosshairState>,
    mut time: ResMut<Time<Virtual>>,
    mut blocks_input: ResMut<BlocksInput>,
) {
    next_menu.set(Menu::None);
    crosshair
        .wants_free_cursor
        .remove(&spawn_pause_menu.type_id());
    blocks_input.remove(&spawn_pause_menu.type_id());
    time.unpause();
}

fn quit_to_title(
    _on: On<Pointer<Click>>,
    mut next_screen: ResMut<NextState<Screen>>,
    mut crosshair: Single<&mut CrosshairState>,
    mut time: ResMut<Time<Virtual>>,
    mut blocks_input: ResMut<BlocksInput>,
) {
    next_screen.set(Screen::Title);
    crosshair
        .wants_free_cursor
        .remove(&spawn_pause_menu.type_id());
    blocks_input.remove(&spawn_pause_menu.type_id());
    time.unpause();
}

fn go_back(
    mut next_menu: ResMut<NextState<Menu>>,
    mut crosshair: Single<&mut CrosshairState>,
    mut time: ResMut<Time<Virtual>>,
    mut blocks_input: ResMut<BlocksInput>,
) {
    next_menu.set(Menu::None);
    crosshair
        .wants_free_cursor
        .remove(&spawn_pause_menu.type_id());
    blocks_input.remove(&spawn_pause_menu.type_id());
    time.unpause();
}
