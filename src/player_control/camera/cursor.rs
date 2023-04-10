use crate::player_control::actions::ActionsFrozen;
use anyhow::{Context, Result};
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};
use bevy_mod_sysfail::macros::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Resource, Serialize, Deserialize, Default)]
pub(crate) struct ForceCursorGrabMode(pub(crate) Option<CursorGrabMode>);

#[sysfail(log(level = "error"))]
pub(crate) fn grab_cursor(
    mut primary_windows: Query<&mut Window, With<PrimaryWindow>>,
    actions_frozen: Res<ActionsFrozen>,
    force_cursor_grab: Res<ForceCursorGrabMode>,
) -> Result<()> {
    #[cfg(feature = "tracing")]
    let _span = info_span!("cursor_grab_system").entered();
    let mut window = primary_windows
        .get_single_mut()
        .context("Failed to get primary window")?;
    let cursor = &mut window.cursor;
    if let Some(mode) = force_cursor_grab.0 {
        cursor.grab_mode = mode;
        cursor.visible = mode != CursorGrabMode::Locked;
    } else if actions_frozen.is_frozen() {
        cursor.grab_mode = CursorGrabMode::None;
        cursor.visible = true;
    } else {
        cursor.grab_mode = CursorGrabMode::Locked;
        cursor.visible = false;
    }
    Ok(())
}
