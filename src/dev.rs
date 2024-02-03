use crate::dev::dev_editor::dev_editor_plugin;
use crate::level_instantiation::spawning::objects::camera::IngameCameraMarker;
use crate::player_control::player_embodiment::Player;
use anyhow::Context;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_editor_pls::egui;
use bevy_editor_pls::prelude::*;
use bevy_egui::EguiContexts;
use bevy_xpbd_3d::prelude::*;
use egui_plot::{Corner, Legend, Line, Plot, PlotPoint, PlotPoints};
use seldom_fn_plugin::FnPluginExt;
use std::collections::VecDeque;

pub(crate) mod dev_editor;

/// Plugin with debugging utility intended for use during development only.
/// Don't include this in a release build.
pub(crate) fn dev_plugin(app: &mut App) {
    {
        app.add_plugins(EditorPlugin::new())
            .insert_resource(default_editor_controls())
            .add_plugins(FrameTimeDiagnosticsPlugin)
            .fn_plugin(dev_editor_plugin)
            .add_plugins(LogDiagnosticsPlugin::filtered(vec![]))
            .add_plugins(PhysicsDebugPlugin::default())
            .insert_resource(PhysicsDebugConfig {
                enabled: false,
                ..default()
            })
            .add_systems(Update, debug);
    }
}

fn default_editor_controls() -> bevy_editor_pls::controls::EditorControls {
    use bevy_editor_pls::controls::*;
    let mut editor_controls = EditorControls::default_bindings();
    editor_controls.unbind(Action::PlayPauseEditor);
    editor_controls.insert(
        Action::PlayPauseEditor,
        Binding {
            input: UserInput::Single(Button::Keyboard(KeyCode::G)),
            conditions: vec![BindingCondition::ListeningForText(false)],
        },
    );
    editor_controls
}

fn debug(
    time: Res<Time<Virtual>>,
    mut egui_contexts: EguiContexts,
    primary_windows: Query<&Window, With<PrimaryWindow>>,
    player: Query<&Transform, (With<Player>, Without<IngameCameraMarker>)>,
    camera: Query<&Transform, With<IngameCameraMarker>>,
    mut player_positions: Local<VecDeque<Vec3>>,
    mut camera_positions: Local<VecDeque<Vec3>>,
    mut time_steps: Local<VecDeque<f32>>,
) {
    let Some(player) = player.iter().next() else {
        return;
    };
    let Some(camera) = camera.iter().next() else {
        return;
    };

    let t = time.elapsed_seconds();
    player_positions.push_back(player.translation);
    camera_positions.push_back(camera.translation);
    time_steps.push_back(t);

    // keep only the positions of the last 1 second
    let max_time = 1.0;
    while let Some(&time) = time_steps.front() {
        if t - time < max_time {
            break;
        }
        time_steps.pop_front();
        player_positions.pop_front();
        camera_positions.pop_front();
    }

    let player_and_time = player_positions
        .iter()
        .cloned()
        .zip(time_steps.iter().cloned());
    let player_deriv = player_and_time
        .clone()
        .zip(player_and_time.skip(1))
        .map(|((p1, t1), (p2, t2))| (p2 - p1) / (t2 - t1));

    let camera_and_time = camera_positions
        .iter()
        .cloned()
        .zip(time_steps.iter().cloned());
    let camera_deriv = camera_and_time
        .clone()
        .zip(camera_and_time.skip(1))
        .map(|((p1, t1), (p2, t2))| (p2 - p1) / (t2 - t1));

    let window = primary_windows
        .get_single()
        .context("Failed to get primary window")
        .unwrap();
    egui::Window::new("Debug Window")
        .auto_sized()
        .fixed_pos(egui::Pos2::new(window.width() * 0.75, 0.))
        .show(egui_contexts.ctx_mut(), |ui| {
            let plot = Plot::new("My Plot")
                .legend(Legend::default().position(Corner::LeftBottom))
                .width(280.0)
                .height(180.0)
                .include_y(-20.0)
                .include_y(20.0);

            plot.show(ui, |ui| {
                let player_line: Vec<PlotPoint> = time_steps
                    .iter()
                    .zip(player_deriv.map(|p| p.length()))
                    .map(|(a, b)| PlotPoint::new(*a, b))
                    .collect();
                let line = Line::new(PlotPoints::Owned(player_line)).name("Player");
                ui.line(line);

                let camera_line: Vec<PlotPoint> = time_steps
                    .iter()
                    .zip(camera_deriv.map(|p| p.length()))
                    .map(|(a, b)| PlotPoint::new(*a, b))
                    .collect();
                let line = Line::new(PlotPoints::Owned(camera_line)).name("Camera");
                ui.line(line);
            });
        });
}
