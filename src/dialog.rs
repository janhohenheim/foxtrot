pub use crate::dialog::resources::{ActiveConditions, DialogEvent};
use crate::dialog::resources::{CurrentDialog, Dialog, DialogId, NextPage};
use crate::GameState;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use std::fs;
use std::path::Path;

mod resources;

pub struct DialogPlugin;
impl Plugin for DialogPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin)
            .init_resource::<ActiveConditions>()
            .add_event::<DialogEvent>()
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(init_dialog))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(set_current_dialog)
                    .with_system(show_dialog),
            );
    }
}

fn init_dialog(mut dialog_event_writer: EventWriter<DialogEvent>) {
    dialog_event_writer.send(DialogEvent(DialogId::new("sample")))
}

fn set_current_dialog(
    mut commands: Commands,
    active_conditions: Res<ActiveConditions>,
    mut dialog_events: EventReader<DialogEvent>,
) {
    for DialogEvent(id) in dialog_events.iter() {
        let dialog = load_dialog(id);
        let starting_page = dialog
            .initial_page
            .iter()
            .filter(|page| page.is_available(&active_conditions))
            .next()
            .unwrap_or_else(|| {
                panic!(
                    "No valid active page for dialog {:?}. Current conditions: {:?}",
                    id, active_conditions
                )
            })
            .id
            .clone();
        commands.insert_resource(CurrentDialog {
            dialog,
            current_page: starting_page,
        });
    }
}

fn show_dialog(
    mut commands: Commands,
    mut current_dialog: Option<ResMut<CurrentDialog>>,
    mut active_conditions: ResMut<ActiveConditions>,
    mut egui_context: ResMut<EguiContext>,
) {
    let mut current_dialog = match current_dialog {
        Some(current_dialog) => current_dialog,
        None => return,
    };
    egui::Window::new("Hello")
        .fixed_size((300., 300.))
        .collapsible(false)
        .fixed_pos((300., 300.))
        .show(egui_context.ctx_mut(), |ui| {
            let current_page = current_dialog.fetch_current_page();
            ui.label(current_page.text.clone());
            present_choices(
                ui,
                &mut commands,
                &mut current_dialog,
                &mut active_conditions,
                current_page.next_page,
            )
        });
}

fn present_choices(
    ui: &mut egui::Ui,
    mut commands: &mut Commands,
    current_dialog: &mut CurrentDialog,
    active_conditions: &mut ActiveConditions,
    next_page: NextPage,
) {
    match next_page {
        NextPage::Continue(next_page_id) => {
            if ui.button("Continue").clicked() {
                current_dialog.current_page = next_page_id.clone();
            }
        }
        NextPage::Choice(choices) => {
            for (choice_id, choice) in choices.iter() {
                if choice.is_available(&active_conditions)
                    && ui.button(choice.text.clone()).clicked()
                {
                    active_conditions.0.insert(choice_id.clone());
                    current_dialog.current_page = choice.next_page_id.clone();
                }
            }
        }
        NextPage::SameAs(other_page_id) => {
            let next_page = current_dialog.fetch_page(&other_page_id).next_page;
            present_choices(ui, commands, current_dialog, active_conditions, next_page);
        }
        NextPage::Exit => {
            if ui.button("Exit").clicked() {
                commands.remove_resource::<CurrentDialog>();
            }
        }
    }
}

fn load_dialog(id: &DialogId) -> Dialog {
    let filename = format!("{}.json", id.0);
    let path = Path::new("assets").join("dialogs").join(filename);
    let json = fs::read_to_string(path.clone())
        .unwrap_or_else(|e| panic!("Failed to open dialog file at {:?}: {}", path, e));
    serde_json::from_str(&json)
        .unwrap_or_else(|e| panic!("Failed to parse dialog file at {:?}: {}", path, e))
}
