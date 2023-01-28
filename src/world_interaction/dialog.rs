use crate::player_control::actions::ActionsFrozen;
use crate::world_interaction::condition::{ActiveConditions, ConditionAddEvent};
pub use crate::world_interaction::dialog::resources::{
    CurrentDialog, Dialog, DialogEvent, DialogId, InitialPage, NextPage,
};
use crate::GameState;
use bevy::prelude::*;
use bevy_egui::egui::Color32;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

mod resources;

pub struct DialogPlugin;
impl Plugin for DialogPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin)
            .register_type::<DialogTarget>()
            .register_type::<DialogId>()
            .add_event::<DialogEvent>()
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(set_current_dialog)
                    .with_system(show_dialog),
            );
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Component, Reflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub struct DialogTarget {
    pub dialog_id: DialogId,
}

fn set_current_dialog(
    mut commands: Commands,
    active_conditions: Res<ActiveConditions>,
    mut dialog_events: EventReader<DialogEvent>,
) {
    for dialog_event in dialog_events.iter() {
        let dialog = load_dialog(&dialog_event.dialog);
        let current_page = dialog_event.page.clone().unwrap_or_else(|| {
            dialog
                .initial_page
                .iter()
                .find(|page| page.is_available(&active_conditions))
                .unwrap_or_else(|| {
                    panic!(
                        "No valid active page for dialog {dialog:?}. Current conditions: {active_conditions:?}"
                    )
                })
                .id
                .clone()
        });
        commands.insert_resource(CurrentDialog {
            id: dialog_event.dialog.clone(),
            dialog,
            current_page,
            last_choice: None,
        });
        commands.init_resource::<ActionsFrozen>();
    }
}

fn show_dialog(
    mut commands: Commands,
    current_dialog: Option<ResMut<CurrentDialog>>,
    active_conditions: Res<ActiveConditions>,
    mut condition_writer: EventWriter<ConditionAddEvent>,
    mut egui_context: ResMut<EguiContext>,
) {
    let mut current_dialog = match current_dialog {
        Some(current_dialog) => current_dialog,
        None => return,
    };
    let height = 150.;
    egui::TopBottomPanel::bottom("Dialog")
        .resizable(false)
        .exact_height(height)
        .frame(egui::Frame {
            fill: Color32::from_black_alpha(250),
            ..default()
        })
        .show(egui_context.ctx_mut(), |ui| {
            let current_page = current_dialog.fetch_current_page();
            ui.vertical_centered_justified(|ui| {
                ui.add_space(5.);
                ui.label(current_page.text.clone());
                ui.add_space(8.);
                present_choices(
                    ui,
                    &mut commands,
                    &mut current_dialog,
                    &active_conditions,
                    &mut condition_writer,
                    current_page.next_page,
                );
                ui.add_space(5.);
            });
        });
}

fn present_choices(
    ui: &mut egui::Ui,
    commands: &mut Commands,
    current_dialog: &mut CurrentDialog,
    active_conditions: &ActiveConditions,
    condition_writer: &mut EventWriter<ConditionAddEvent>,
    next_page: NextPage,
) {
    match next_page {
        NextPage::Continue(next_page_id) => {
            if ui.button("Continue").clicked() {
                current_dialog.current_page = next_page_id;
            }
        }
        NextPage::Choice(choices) => {
            for (choice_id, choice) in choices.iter() {
                let was_just_picked = current_dialog
                    .last_choice
                    .as_ref()
                    .map(|id| id == choice_id)
                    .unwrap_or_default();
                if choice.is_available(active_conditions)
                    && !was_just_picked
                    && ui.button(choice.text.clone()).clicked()
                {
                    condition_writer.send(ConditionAddEvent(choice_id.clone()));
                    current_dialog.last_choice = Some(choice_id.clone());
                    current_dialog.current_page = choice.next_page_id.clone();
                }
            }
        }
        NextPage::SameAs(other_page_id) => {
            let next_page = current_dialog.fetch_page(&other_page_id).next_page;
            present_choices(
                ui,
                commands,
                current_dialog,
                active_conditions,
                condition_writer,
                next_page,
            );
        }
        NextPage::Exit => {
            if ui.button("Exit").clicked() {
                commands.remove_resource::<CurrentDialog>();
                commands.remove_resource::<ActionsFrozen>();
            }
        }
    }
}

fn load_dialog(id: &DialogId) -> Dialog {
    let filename = format!("{}.ron", id.0);
    let path = Path::new("assets").join("dialogs").join(filename);
    let serialized = fs::read_to_string(path.clone())
        .unwrap_or_else(|e| panic!("Failed to open dialog file at {path:?}: {e}"));
    ron::from_str(&serialized)
        .unwrap_or_else(|e| panic!("Failed to parse dialog file at {path:?}: {e}"))
}
