use bevy::prelude::*;
use bevy_trenchbroom::prelude::*;

use crate::third_party::bevy_trenchbroom::{BrushEntitySpawned, NotifyBrushEntitySpawned};

use super::effects::prepare_light_meshes;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<LightWindow>();
    app.add_observer(setup_light_window_brush_entity);
}

#[derive(SolidClass, Component, Debug, Reflect)]
#[reflect(QuakeClass, Component)]
#[base(Transform, Visibility)]
#[geometry(GeometryProvider::new().trimesh_collider().smooth_by_default_angle().with_lightmaps())]
pub(crate) struct LightWindow;

fn setup_light_window_brush_entity(trigger: Trigger<OnAdd, LightWindow>, mut commands: Commands) {
    let entity = trigger.target();
    commands
        .entity(entity)
        .insert(NotifyBrushEntitySpawned)
        .observe(setup_light_window_brushes);
}

fn setup_light_window_brushes(
    trigger: Trigger<BrushEntitySpawned>,
    children: Query<&Children>,
    mut commands: Commands,
) {
    let entity = trigger.target();
    let Ok(brushes) = children.get(entity) else {
        return;
    };
    for brush_entity in brushes.iter() {
        commands
            .entity(brush_entity)
            .insert(children![(
                SpotLight {
                    color: Color::srgb_u8(239, 173, 144),
                    intensity: 200_000.0,
                    radius: 10.0,
                    shadows_enabled: true,
                    #[cfg(feature = "native")]
                    soft_shadows_enabled: true,
                    ..default()
                },
                Transform::IDENTITY.looking_to(Vec3::X, Vec3::Y),
            )])
            .queue(prepare_light_meshes);
    }
}
