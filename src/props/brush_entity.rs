use bevy::prelude::*;
use bevy_trenchbroom::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<LightWindow>();
    app.add_observer(setup_light_window);
}

#[derive(SolidClass, Component, Debug, Reflect)]
#[reflect(QuakeClass, Component)]
#[base(Transform, Visibility)]
#[geometry(GeometryProvider::new().trimesh_collider().smooth_by_default_angle().with_lightmaps())]
pub(crate) struct LightWindow;

fn setup_light_window(trigger: Trigger<OnAdd, LightWindow>, mut commands: Commands) {
    let entity = trigger.target();
    commands.entity(entity).insert(children![(
        PointLight {
            color: Color::WHITE,
            intensity: 20_000.0,
            radius: 10.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::IDENTITY.looking_to(Vec3::X, Vec3::Y),
    )]);
}
