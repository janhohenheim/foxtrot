//! The main menu (seen on the title screen).
use bevy::{prelude::*, window::CursorGrabMode};

use crate::{
    menus::Menu,
    screens::Screen,
    theme::{palette::SCREEN_BACKGROUND, widget},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Main), spawn_main_menu);
}

fn spawn_main_menu(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Main Menu"),
        BackgroundColor(SCREEN_BACKGROUND),
        GlobalZIndex(2),
        StateScoped(Menu::Main),
        #[cfg(not(target_family = "wasm"))]
        children![
            widget::button("Play", enter_loading_screen),
            widget::button("Settings", open_settings_menu),
            widget::button("Credits", open_credits_menu),
            widget::button("Exit", exit_app),
        ],
        #[cfg(target_family = "wasm")]
        children![
            widget::button("Play", enter_loading_screen),
            widget::button("Settings", open_settings_menu),
            widget::button("Credits", open_credits_menu),
        ],
    ));
}

fn enter_loading_screen(
    _trigger: Trigger<Pointer<Click>>,
    mut next_screen: ResMut<NextState<Screen>>,
    mut window: Single<&mut Window>,
) {
    next_screen.set(Screen::Loading);
    window.cursor_options.grab_mode = CursorGrabMode::Locked;
}

fn open_settings_menu(_: Trigger<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Settings);
}

fn open_credits_menu(_: Trigger<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Credits);
}

#[cfg(not(target_family = "wasm"))]
fn exit_app(_: Trigger<Pointer<Click>>, mut app_exit: EventWriter<AppExit>) {
    app_exit.write(AppExit::Success);
}
