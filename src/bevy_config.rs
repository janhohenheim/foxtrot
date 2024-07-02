use crate::util::error;
use bevy::core::FrameCount;
use bevy::{prelude::*, window::PrimaryWindow, winit::WinitWindows};
use std::io::Cursor;
use winit::window::Icon;

/// Overrides the default Bevy plugins and configures things like the screen settings.
pub(super) fn plugin(app: &mut App) {
    app.add_plugins(
        DefaultPlugins.set(WindowPlugin {
            primary_window: Window {
                title: "Foxtrot".to_string(),
                // This will spawn an invisible window
                // The window will be made visible in the make_visible() system after 3 frames.
                // This is useful when you want to avoid the white window that shows up before the GPU is ready to render the app.
                visible: false,
                ..default()
            }
            .into(),
            ..default()
        }),
    )
    .insert_resource(Msaa::Sample4)
    .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
    .add_systems(Startup, set_window_icon.pipe(error))
    .add_systems(Update, make_visible);
}

// Sets the icon on Windows and X11
fn set_window_icon(
    windows: NonSend<WinitWindows>,
    primary_windows: Query<Entity, With<PrimaryWindow>>,
) -> anyhow::Result<()> {
    let primary_entity = primary_windows.single();
    let Some(primary) = windows.get_window(primary_entity) else {
        return Ok(());
    };
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
    Ok(())
}

/// Source: <https://github.com/bevyengine/bevy/blob/v0.14.0-rc.4/examples/window/window_settings.rs#L56>
fn make_visible(mut window: Query<&mut Window>, frames: Res<FrameCount>) {
    // The delay may be different for your app or system.
    if frames.0 == 3 {
        // At this point the gpu is ready to show the app so we can make the window visible.
        // Alternatively, you could toggle the visibility in Startup.
        // It will work, but it will have one white frame before it starts rendering
        window.single_mut().visible = true;
    }
}
