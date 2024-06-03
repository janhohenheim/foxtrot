use anyhow::Context;
use bevy::render::settings::{WgpuFeatures, WgpuSettings};
use bevy::render::RenderPlugin;
use bevy::{prelude::*, window::PrimaryWindow, winit::WinitWindows};
use bevy_mod_sysfail::prelude::*;
use std::io::Cursor;
use winit::window::Icon;

/// Overrides the default Bevy plugins and configures things like the screen settings.
pub(super) fn plugin(app: &mut App) {
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Foxtrot".to_string(),
                    ..default()
                }),
                ..default()
            })
            .set(RenderPlugin {
                render_creation: create_wgpu_settings().into(),
                synchronous_pipeline_compilation: false,
            }),
    )
    .insert_resource(Msaa::Sample4)
    .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
    .add_systems(Startup, set_window_icon);
}

fn create_wgpu_settings() -> WgpuSettings {
    let mut wgpu_settings = WgpuSettings::default();
    wgpu_settings
        .features
        .set(WgpuFeatures::VERTEX_WRITABLE_STORAGE, true);
    wgpu_settings
}

// Sets the icon on Windows and X11
#[sysfail(Log<anyhow::Error, Error>)]
fn set_window_icon(
    windows: NonSend<WinitWindows>,
    primary_windows: Query<Entity, With<PrimaryWindow>>,
) {
    let primary_entity = primary_windows.single();
    let primary = windows
        .get_window(primary_entity)
        .context("Failed to get primary window")?;
    let icon_buf = Cursor::new(include_bytes!(
        "../build/macos/AppIcon.iconset/icon_256x256.png"
    ));
    if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
        let image = image.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        let icon = Icon::from_rgba(rgba, width, height)?;
        primary.set_window_icon(Some(icon));
    };
}
