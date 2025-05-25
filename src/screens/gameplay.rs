//! The screen state for the main gameplay.

use bevy::{input::common_conditions::input_just_pressed, prelude::*};
#[cfg(feature = "hot_patch")]
use bevy_simple_subsecond_system::hot;

use crate::{PostPhysicsAppSystems, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        return_to_title_screen
            .run_if(in_state(Screen::Gameplay).and(input_just_pressed(KeyCode::Escape)))
            .in_set(PostPhysicsAppSystems::Update),
    );
}

#[cfg_attr(feature = "hot_patch", hot)]
fn return_to_title_screen(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}
