use crate::file_system_interaction::asset_loading::DialogAssets;
use crate::file_system_interaction::config::GameConfig;
use crate::player_control::actions::{ActionsFrozen, PlayerAction};
use crate::world_interaction::condition::{ActiveConditions, ConditionAddEvent, ConditionId};
use crate::world_interaction::dialog::resources::Page;
pub(crate) use crate::world_interaction::dialog::resources::{
    CurrentDialog, Dialog, DialogEvent, DialogId, NextPage,
};
use crate::GameState;
use anyhow::{Context, Ok, Result};
use bevy::prelude::*;
use bevy_egui::egui::FontFamily::Proportional;
use bevy_egui::egui::FontId;
use bevy_egui::egui::TextStyle::{Body, Button};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_mod_sysfail::macros::*;
use leafwing_input_manager::prelude::ActionState;
use serde::{Deserialize, Serialize};
use std::path::Path;
use unicode_segmentation::UnicodeSegmentation;

mod resources;

pub(crate) fn dialog_plugin(app: &mut App) {
    app.add_plugin(EguiPlugin)
        .register_type::<DialogId>()
        .add_event::<DialogEvent>()
        .add_systems((set_current_dialog, show_dialog).in_set(OnUpdate(GameState::Playing)));
}

#[derive(Debug, Clone, Eq, PartialEq, Component, Serialize, Deserialize, Default)]
pub(crate) struct DialogTarget {
    pub(crate) dialog_id: DialogId,
}

#[sysfail(log(level = "error"))]
fn set_current_dialog(
    mut commands: Commands,
    active_conditions: Res<ActiveConditions>,
    mut dialog_events: EventReader<DialogEvent>,
    dialogs: Res<Assets<Dialog>>,
    dialog_handles: Res<DialogAssets>,
    mut actions_frozen: ResMut<ActionsFrozen>,
) -> Result<()> {
    for dialog_event in dialog_events.iter() {
        let path = Path::new("dialogs")
            .join(&dialog_event.dialog.0.clone())
            .with_extension("dlg.ron")
            .to_str()
            .with_context(|| {
                format!(
                    "Failed to convert dialog path to string for dialog: {:?}",
                    dialog_event.dialog
                )
            })?
            .to_owned();
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

#[sysfail(log(level = "error"))]
fn show_dialog(
    mut commands: Commands,
    current_dialog: Option<ResMut<CurrentDialog>>,
    active_conditions: Res<ActiveConditions>,
    mut condition_writer: EventWriter<ConditionAddEvent>,
    mut egui_contexts: EguiContexts,
    mut actions_frozen: ResMut<ActionsFrozen>,
    actions: Query<&ActionState<PlayerAction>>,
    time: Res<Time>,
    mut elapsed_time: Local<f32>,
    config: Res<GameConfig>,
) -> Result<()> {
    let Some(mut current_dialog) = current_dialog else {
            *elapsed_time = 0.0;
            return Ok(());
    };

    for actions in actions.iter() {
        let current_page = current_dialog.fetch_current_page()?;
        get_dialog_window()
            .show(egui_contexts.ctx_mut(), |ui| {
                // Get current context style
                set_dialog_style(ui.style_mut());
                let dialog_size = egui::Vec2::new(500., 150.);
                ui.set_width(dialog_size.x);
                ui.set_height(dialog_size.y);

                let dialog_text = create_dialog_rich_text(&current_page, *elapsed_time, &config);
                ui.vertical(|ui| {
                    ui.add_space(5.);
                    ui.label(&dialog_text);
                    if dialog_text == current_page.text {
                        ui.add_space(3.);
                        ui.separator();
                        ui.add_space(8.);
                        present_choices(
                            ui,
                            &mut commands,
                            &mut current_dialog,
                            &active_conditions,
                            &mut condition_writer,
                            &mut actions_frozen,
                            actions,
                            current_page.next_page,
                            &mut elapsed_time,
                        )
                        .context("Failed to present dialog choices")?;
                    }
                    Ok(())
                })
                .inner
            })
            .context("Failed to show dialog window")?
            .inner
            .context("Failed to fetch inner result when showing dialog window")??;
        let dt_speed_multiplier = if actions.pressed(PlayerAction::SpeedUpDialog) {
            4.
        } else {
            1.
        };
        *elapsed_time += time.delta_seconds() * dt_speed_multiplier;
    }
    Ok(())
}

fn present_choices(
    ui: &mut egui::Ui,
    commands: &mut Commands,
    current_dialog: &mut CurrentDialog,
    active_conditions: &ActiveConditions,
    condition_writer: &mut EventWriter<ConditionAddEvent>,
    actions_frozen: &mut ActionsFrozen,
    actions: &ActionState<PlayerAction>,
    next_page: NextPage,
    elapsed_time: &mut f32,
) -> Result<()> {
    match next_page {
        NextPage::Continue(next_page_id) => {
            let text = create_choice_rich_text(0, "Continue");
            if ui.button(text).clicked() || actions.just_pressed(PlayerAction::numbered_choice(1)) {
                current_dialog.current_page = next_page_id;
                *elapsed_time = 0.0;
            }
        }
        NextPage::Choice(choices) => {
            let mut picked_choice = None;
            for (index, (choice_id, choice)) in choices
                .iter()
                .filter(|(choice_id, choice)| {
                    choice.is_available(active_conditions)
                        && !was_just_picked(current_dialog, choice_id)
                })
                .enumerate()
            {
                let text = create_choice_rich_text(index, &choice.text);
                if ui.button(&text).clicked()
                    || actions.just_pressed(PlayerAction::numbered_choice(index as u8 + 1))
                {
                    picked_choice = Some((choice_id.clone(), choice.clone()));
                }
            }
            if let Some((choice_id, choice)) = picked_choice {
                condition_writer.send(ConditionAddEvent(choice_id.clone()));
                current_dialog.last_choice = Some(choice_id);
                current_dialog.current_page = choice.next_page_id;
                *elapsed_time = 0.0;
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
                elapsed_time,
            )?;
        }
        NextPage::Exit => {
            let text = create_choice_rich_text(0, "Exit");
            if ui.button(text).clicked() || actions.just_pressed(PlayerAction::numbered_choice(1)) {
                commands.remove_resource::<CurrentDialog>();
                actions_frozen.unfreeze();
            }
        }
    }
    Ok(())
}

fn get_dialog_window() -> egui::Window<'static> {
    egui::Window::new("Dialog")
        .anchor(egui::Align2::CENTER_BOTTOM, egui::Vec2::new(0., -30.))
        .collapsible(false)
        .resizable(false)
        .title_bar(false)
        .frame(egui::Frame {
            fill: egui::Color32::from_black_alpha(230),
            inner_margin: egui::style::Margin::same(25.),
            rounding: egui::Rounding::same(30.0),
            ..default()
        })
}

fn set_dialog_style(style: &mut egui::Style) {
    style.text_styles = [
        (Body, FontId::new(16.0, Proportional)),
        (Button, FontId::new(14.0, Proportional)),
    ]
    .into();
    style.visuals.button_frame = false;
    style.visuals.widgets.noninteractive.fg_stroke.color = egui::Color32::from_gray(250);
}

fn create_dialog_rich_text(page: &Page, elapsed_time: f32, config: &GameConfig) -> String {
    let base_letters_per_second = config.dialog.base_letters_per_second;
    let letters_to_display = (base_letters_per_second * page.talking_speed * elapsed_time) as usize;
    page.text.graphemes(true).take(letters_to_display).collect()
}

fn create_choice_rich_text(index: usize, text: &str) -> String {
    format!("{}. {}", index + 1, text)
}

fn was_just_picked(current_dialog: &CurrentDialog, choice_id: &ConditionId) -> bool {
    current_dialog
        .last_choice
        .as_ref()
        .map(|id| id == choice_id)
        .unwrap_or_default()
}
