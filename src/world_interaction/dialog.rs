use crate::file_system_interaction::asset_loading::DialogAssets;
use crate::player_control::actions::{Actions, ActionsFrozen, UiActions};
use crate::util::log_error::log_errors;
use crate::world_interaction::condition::{ActiveConditions, ConditionAddEvent};
pub use crate::world_interaction::dialog::resources::{
    CurrentDialog, Dialog, DialogEvent, DialogId, InitialPage, NextPage,
};
use crate::GameState;
use anyhow::{Context, Result};
use bevy::prelude::*;
use bevy_egui::egui::Color32;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use serde::{Deserialize, Serialize};

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
                    .with_system(set_current_dialog.pipe(log_errors))
                    .with_system(show_dialog.pipe(log_errors)),
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
    dialogs: Res<Assets<Dialog>>,
    dialog_handles: Res<DialogAssets>,
    mut actions_frozen: ResMut<ActionsFrozen>,
) -> Result<()> {
    for dialog_event in dialog_events.iter() {
        let path = format!("dialogs/{}.dlg.ron", dialog_event.dialog.0);
        let dialog_handle = match dialog_handles.dialogs.get(&path) {
            Some(handle) => handle,
            None => {
                error!(
                    "Failed to load dialog \"{}\": No such dialog. Available dialog: {:?}",
                    path,
                    dialog_handles.dialogs.keys()
                );
                continue;
            }
        };
        let dialog = dialogs
            .get(dialog_handle)
            .context("Failed to get dialog handle in dialog assets")?;
        let current_page = dialog_event.page.clone().or_else(|| {
            dialog
                .initial_page
                .iter()
                .find(|page| page.is_available(&active_conditions))
                ?
                .id
                .clone()
                .into()
        }).with_context(|| {
            format!(
                "No valid active page for dialog {dialog:?}. Current conditions: {active_conditions:?}"
            )
        })?;
        commands.insert_resource(CurrentDialog {
            source: dialog_event.source,
            id: dialog_event.dialog.clone(),
            dialog: dialog.clone(),
            current_page,
            last_choice: None,
        });
        actions_frozen.freeze();
    }
    Ok(())
}

fn show_dialog(
    mut commands: Commands,
    current_dialog: Option<ResMut<CurrentDialog>>,
    active_conditions: Res<ActiveConditions>,
    mut condition_writer: EventWriter<ConditionAddEvent>,
    mut egui_context: ResMut<EguiContext>,
    mut actions_frozen: ResMut<ActionsFrozen>,
    actions: Res<Actions>,
) -> Result<()> {
    let mut current_dialog = match current_dialog {
        Some(current_dialog) => current_dialog,
        None => return Ok(()),
    };
    let height = 150.;
    let current_page = current_dialog.fetch_current_page()?;
    egui::TopBottomPanel::bottom("Dialog")
        .resizable(false)
        .exact_height(height)
        .frame(egui::Frame {
            fill: Color32::from_black_alpha(250),
            ..default()
        })
        .show(egui_context.ctx_mut(), |ui| {
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
                    &mut actions_frozen,
                    &actions.ui,
                    current_page.next_page,
                )
                .expect("Failed to present dialog choices");
                ui.add_space(5.);
            });
        });
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn present_choices(
    ui: &mut egui::Ui,
    commands: &mut Commands,
    current_dialog: &mut CurrentDialog,
    active_conditions: &ActiveConditions,
    condition_writer: &mut EventWriter<ConditionAddEvent>,
    actions_frozen: &mut ActionsFrozen,
    actions: &UiActions,
    next_page: NextPage,
) -> Result<()> {
    match next_page {
        NextPage::Continue(next_page_id) => {
            if ui.button("1. Continue").clicked() || actions.numbered_choice[1] {
                current_dialog.current_page = next_page_id;
            }
        }
        NextPage::Choice(choices) => {
            for (index, (choice_id, choice)) in choices.iter().enumerate() {
                let index = index + 1;
                let was_just_picked = current_dialog
                    .last_choice
                    .as_ref()
                    .map(|id| id == choice_id)
                    .unwrap_or_default();
                let text = format!("{}. {}", index, choice.text);
                if choice.is_available(active_conditions)
                    && !was_just_picked
                    && (ui.button(text).clicked() || actions.numbered_choice[index])
                {
                    condition_writer.send(ConditionAddEvent(choice_id.clone()));
                    current_dialog.last_choice = Some(choice_id.clone());
                    current_dialog.current_page = choice.next_page_id.clone();
                }
            }
        }
        NextPage::SameAs(other_page_id) => {
            let next_page = current_dialog.fetch_page(&other_page_id)?.next_page;
            present_choices(
                ui,
                commands,
                current_dialog,
                active_conditions,
                condition_writer,
                actions_frozen,
                actions,
                next_page,
            )?;
        }
        NextPage::Exit => {
            if ui.button("1. Exit").clicked() || actions.numbered_choice[1] {
                commands.remove_resource::<CurrentDialog>();
                actions_frozen.unfreeze();
            }
        }
    }
    Ok(())
}
