use bevy::prelude::*;
use bevy_trenchbroom::prelude::*;

use crate::props::effects::disable_shadow_casting;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<LightWindow>();
    app.add_observer(setup_light_window_brush_entity);
}

#[derive(SolidClass, Component, Debug, Default, Reflect)]
#[reflect(QuakeClass, Component)]
#[base(Transform, Visibility)]
#[geometry(GeometryProvider::new().trimesh_collider().smooth_by_default_angle())]
pub(crate) struct LightWindow {
    angles: Vec3,
}

fn setup_light_window_brush_entity(
    trigger: Trigger<OnAdd, LightWindow>,
    brush_entity: Query<(&LightWindow, &Children)>,
    mut commands: Commands,
) {
    let entity = trigger.target();
    let Ok((light_window, children)) = brush_entity.get(entity) else {
        return;
    };
    let rotation = Quat::from_euler(
        EulerRot::XYZ,
        light_window.angles.x.to_radians(),
        light_window.angles.y.to_radians(),
        light_window.angles.z.to_radians(),
    );
    for brush in children.iter() {
        commands
            .entity(brush)
            .insert(children![(
                SpotLight {
                    color: Color::srgb_u8(239, 173, 144),
                    intensity: 200_000.0,
                    radius: 0.1,
                    shadows_enabled: true,
                    #[cfg(feature = "native")]
                    soft_shadows_enabled: true,
                    ..default()
                },
                Transform::IDENTITY.with_rotation(rotation),
            )])
            .queue(disable_shadow_casting);
    }
}
