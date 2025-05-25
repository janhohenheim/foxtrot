use bevy::prelude::*;
#[cfg(feature = "hot_patch")]
use bevy_simple_subsecond_system::hot;
use bevy_trenchbroom::prelude::*;

use crate::props::effects::disable_shadow_casting;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<LightWindow>();
    app.add_observer(setup_light_window_brush_entity);
}

#[derive(SolidClass, Component, Debug, Default, Reflect)]
#[reflect(QuakeClass, Component)]
#[base(Transform, Visibility)]
#[spawn_hooks(SpawnHooks::new().convex_collider().smooth_by_default_angle())]
pub(crate) struct LightWindow;

#[cfg_attr(feature = "hot_patch", hot)]
fn setup_light_window_brush_entity(trigger: Trigger<OnAdd, LightWindow>, mut commands: Commands) {
    let entity = trigger.target();
    commands
        .entity(entity)
        // Using `children!` here would run into https://github.com/Noxmore/bevy_trenchbroom/issues/95
        .with_child(SpotLight {
            color: Color::srgb_u8(239, 173, 144),
            intensity: 200_000.0,
            radius: 0.1,
            shadows_enabled: true,
            #[cfg(feature = "native")]
            soft_shadows_enabled: true,
            ..default()
        })
        .queue(disable_shadow_casting);
}
