use bevy::{
    ecs::{component::ComponentId, world::DeferredWorld},
    prelude::*,
    render::view::RenderLayers,
};
use bevy_hanabi::prelude::*;
use bevy_trenchbroom::util::IsSceneWorld as _;

use crate::{
    RenderLayer,
    props::{BurningLogs, generic::static_bundle},
};

pub(super) fn plugin(_app: &mut App) {}

pub(crate) fn setup_burning_logs(mut world: DeferredWorld, entity: Entity, _id: ComponentId) {
    if world.is_scene_world() {
        return;
    }
    let bundle = static_bundle::<BurningLogs>(&world);
    let effect_handle = setup(&mut world.resource_mut::<Assets<EffectAsset>>());
    world.commands().entity(entity).insert((
        bundle,
        ParticleEffect::new(effect_handle),
        RenderLayers::from(RenderLayer::PARTICLES),
    ));
}

fn setup(effects: &mut Assets<EffectAsset>) -> Handle<EffectAsset> {
    // Color gradient for fire: bright yellow → orange → dark red → transparent
    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::new(1.0, 0.8, 0.0, 1.0)); // bright yellow
    gradient.add_key(0.3, Vec4::new(1.0, 0.4, 0.0, 1.0)); // orange
    gradient.add_key(0.6, Vec4::new(0.6, 0.0, 0.0, 0.8)); // dark red
    gradient.add_key(1.0, Vec4::new(0.0, 0.0, 0.0, 0.0)); // transparent

    let writer = ExprWriter::new();

    // Random upward velocity with some lateral randomness for flicker
    let vx = writer.lit(-0.5).uniform(writer.lit(0.5));
    let vy = writer.lit(3.0).uniform(writer.lit(6.0));
    let vz = writer.lit(-0.5).uniform(writer.lit(0.5));
    let velocity = vx.vec3(vy, vz);

    let mut module = writer.finish();

    let init_vel = SetAttributeModifier::new(Attribute::VELOCITY, velocity.expr());

    // Spawn from small spherical area at the base
    let init_pos = SetPositionSphereModifier {
        center: module.lit(Vec3::ZERO),
        radius: module.lit(0.2),
        dimension: ShapeDimension::Volume,
    };

    // Short lifetime for fire particles
    let lifetime = module.lit(1.0);
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    // Constant upward acceleration (mimics heat rise)
    let accel = module.lit(Vec3::new(0.0, 1.0, 0.0));
    let update_accel = AccelModifier::new(accel);

    const MAX_PARTICLES: u32 = 32768;
    let effect = EffectAsset::new(MAX_PARTICLES, SpawnerSettings::rate(100.0.into()), module)
        .with_name("FireEffect")
        .init(init_pos)
        .init(init_vel)
        .init(init_lifetime)
        .update(update_accel)
        .render(ColorOverLifetimeModifier {
            gradient,
            ..default()
        });

    effects.add(effect)
}
