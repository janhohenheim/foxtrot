use crate::util::log_error::log_errors;
use crate::GameState;
use anyhow::Ok;
use anyhow::Result;
use bevy::prelude::*;
use bevy_egui::egui::FontFamily::Proportional;
use bevy_egui::egui::FontId;
use bevy_egui::egui::TextStyle::{Button, Heading};
use bevy_egui::{egui, EguiContext};

/// This plugin is responsible for the game menu
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited.
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Menu).with_system(setup_menu.pipe(log_errors)),
        );
    }
}

fn setup_menu(
    mut egui_context: ResMut<EguiContext>,
    mut state: ResMut<State<GameState>>,
) -> Result<()> {
    get_menu_panel()
        .show(egui_context.ctx_mut(), |ui| {
            set_menu_style(ui.style_mut());
            ui.vertical_centered_justified(|ui| {
                ui.add_space(50.);
                ui.heading("Foxtrot");
                ui.separator();
                ui.add_space(50.);
                if ui.button("Play").clicked() {
                    state.set(GameState::Playing)?;
                }
                Ok(())
            })
            .inner
        })
        .inner?;
    Ok(())
}

fn get_menu_panel() -> egui::CentralPanel {
    egui::CentralPanel::default().frame(egui::Frame {
        inner_margin: egui::style::Margin::same(60.),
        ..default()
    })
}

fn set_menu_style(style: &mut egui::Style) {
    style.text_styles = [
        (Heading, FontId::new(30.0, Proportional)),
        (Button, FontId::new(20.0, Proportional)),
    ]
    .into();
    style.visuals.widgets.noninteractive.fg_stroke.color = egui::Color32::from_gray(250);
}
