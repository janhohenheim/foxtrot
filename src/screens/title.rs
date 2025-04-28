//! The title screen that appears when the game starts.

use bevy::prelude::*;

use crate::{
    asset_tracking::ResourceHandles,
    screens::Screen,
    theme::{interaction::OnPress, palette::SCREEN_BACKGROUND, prelude::*},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Title), spawn_title_screen);
}

fn spawn_title_screen(mut commands: Commands) {
    commands
        .ui_root()
        .insert((
            StateScoped(Screen::Title),
            BackgroundColor(SCREEN_BACKGROUND),
        ))
        .with_children(|children| {
            children
                .button("Play")
                .observe(enter_loading_or_spawn_screen);
            children.button("Credits").observe(enter_credits_screen);

            #[cfg(not(target_family = "wasm"))]
            children.button("Exit").observe(exit_app);
        });
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

fn enter_credits_screen(_trigger: Trigger<OnPress>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Credits);
}

#[cfg(not(target_family = "wasm"))]
fn exit_app(_trigger: Trigger<OnPress>, mut app_exit: EventWriter<AppExit>) {
    app_exit.send(AppExit::Success);
}
