use crate::player_control::actions::{ActionsFrozen, UiAction};
use crate::GameState;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use leafwing_input_manager::prelude::ActionState;

/// Handles the pause menu accessed while playing the game via ESC.
pub struct IngameMenuPlugin;

impl Plugin for IngameMenuPlugin {
    fn build(&self, app: &mut App) {
        {
            app.add_system_set(SystemSet::on_update(GameState::Playing).with_system(handle_pause));
        }
    }
}

fn handle_pause(
    mut time: ResMut<Time>,
    actions: Query<&ActionState<UiAction>>,
    mut actions_frozen: ResMut<ActionsFrozen>,
    mut egui_context: ResMut<EguiContext>,
    mut paused: Local<bool>,
) {
    for action in actions.iter() {
        let toggled = action.just_pressed(UiAction::TogglePause);
        if *paused {
            if toggled {
                *paused = false;
                time.unpause();
                actions_frozen.unfreeze();
            } else {
                egui::CentralPanel::default()
                    .frame(egui::Frame {
                        fill: egui::Color32::from_black_alpha(240),
                        ..default()
                    })
                    .show(egui_context.ctx_mut(), |ui| {
                        ui.vertical_centered_justified(|ui| {
                            ui.visuals_mut().override_text_color =
                                Some(egui::Color32::from_gray(240));
                            ui.add_space(100.0);
                            ui.heading("Game Paused");
                            ui.separator();
                            ui.label("Press ESC to resume");
                        });
                    });
            }
        } else if toggled {
            *paused = true;
            time.pause();
            actions_frozen.freeze();
        }
    }
}
