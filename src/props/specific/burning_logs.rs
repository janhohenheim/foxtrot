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
    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::new(1., 0., 0., 1.));
    gradient.add_key(1.0, Vec4::splat(0.));

    let writer = ExprWriter::new();
    let zero = writer.lit(0.);
    let y = writer.lit(3.).uniform(writer.lit(10.));
    let v = zero.clone().vec3(y, zero);
    let init_vel = SetAttributeModifier::new(Attribute::VELOCITY, v.expr());

    let mut module = writer.finish();

    let init_pos = SetPositionSphereModifier {
        center: module.lit(Vec3::ZERO),
        radius: module.lit(0.5),
        dimension: ShapeDimension::Surface,
    };

    let lifetime = module.lit(10.);
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    let accel = module.lit(Vec3::new(0., 1., 0.));
    let update_accel = AccelModifier::new(accel);

    const MAX_PARTICLES: u32 = 32768;
    let effect = EffectAsset::new(MAX_PARTICLES, SpawnerSettings::rate(5.0.into()), module)
        .with_name("BurningLogs")
        .init(init_pos)
        .init(init_vel)
        .init(init_lifetime)
        .update(update_accel)
        // Render the particles with a color gradient over their
        // lifetime. This maps the gradient key 0 to the particle spawn
        // time, and the gradient key 1 to the particle death (10s).
        .render(ColorOverLifetimeModifier {
            gradient,
            blend: ColorBlendMode::Add,
            ..default()
        });

    // Insert into the asset system
    effects.add(effect)
}
