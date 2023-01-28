use bevy::prelude::*;
use bevy::window::PresentMode;

pub struct BevyConfigPlugin;

impl Plugin for BevyConfigPlugin {
    fn build(&self, app: &mut App) {
        let default_plugins = DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: 800.,
                height: 600.,
                title: "Foxtrot".to_string(), // ToDo
                canvas: Some("#bevy".to_owned()),
                present_mode: PresentMode::AutoVsync,
                ..default()
            },
            ..default()
        });
        #[cfg(feature = "dev")]
        let default_plugins = default_plugins.set(AssetPlugin {
            watch_for_changes: true,
            ..default()
        });
        app.insert_resource(Msaa { samples: 4 })
            .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
            .add_plugins(default_plugins);
    }
}
