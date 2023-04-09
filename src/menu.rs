use crate::GameState;
use bevy::prelude::*;
use bevy_egui::egui::FontFamily::Proportional;
use bevy_egui::egui::FontId;
use bevy_egui::egui::TextStyle::{Button, Heading};
use bevy_egui::{egui, EguiContexts};

/// This plugin is responsible for the game menu
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited.
pub(crate) fn menu_plugin(app: &mut App) {
    app.add_system(setup_menu.in_set(OnUpdate(GameState::Menu)));
}

fn setup_menu(mut egui_contexts: EguiContexts, mut next_state: ResMut<NextState<GameState>>) {
    get_menu_panel().show(egui_contexts.ctx_mut(), |ui| {
        set_menu_style(ui.style_mut());
        ui.vertical_centered_justified(|ui| {
            ui.add_space(50.);
            ui.heading("Foxtrot");
            ui.separator();
            ui.add_space(50.);
            if ui.button("Play").clicked() {
                next_state.set(GameState::Playing);
            }
        })
    });
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
