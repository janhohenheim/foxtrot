use std::f32::consts::TAU;

use bevy::{
    audio::{SpatialScale, Volume},
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
    let circle: Handle<Image> = world.resource_mut::<AssetServer>().load("images/Flame.png");
    let sound_effect: Handle<AudioSource> = world
        .resource_mut::<AssetServer>()
        .load("audio/music/loop_flames_03.ogg");
    world.commands().entity(entity).insert((
        bundle,
        ParticleEffect::new(effect_handle),
        RenderLayers::from(RenderLayer::PARTICLES),
        EffectMaterial {
            images: vec![circle.clone()],
        },
        AudioPlayer(sound_effect.clone()),
        PlaybackSettings::LOOP
            .with_spatial(true)
            .with_volume(Volume::new(0.25))
            .with_spatial_scale(SpatialScale::new(0.3)),
    ));
}

fn setup(effects: &mut Assets<EffectAsset>) -> Handle<EffectAsset> {
    // Color gradient for fire: bright yellow → orange → dark red → transparent
    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::new(1.0, 0.8, 0.0, 1.0)); // bright yellow
    gradient.add_key(0.3, Vec4::new(1.0, 0.4, 0.0, 1.0)); // orange
    gradient.add_key(0.6, Vec4::new(0.6, 0.0, 0.0, 0.8)); // dark red
    gradient.add_key(1.0, Vec4::new(0.0, 0.0, 0.0, 0.0)); // transparent
    let color_over_lifetime = ColorOverLifetimeModifier {
        gradient,
        ..default()
    };

    let writer = ExprWriter::new();

    // Random upward velocity with some lateral randomness for flicker
    const LATERAL_EXTENT: f32 = 0.3;
    let min_velocity = writer.lit(Vec3::new(-LATERAL_EXTENT, 1.0, -LATERAL_EXTENT));
    let max_velocity = writer.lit(Vec3::new(LATERAL_EXTENT, 2.0, LATERAL_EXTENT));
    let velocity = min_velocity.uniform(max_velocity);

    // Load the texture
    let particle_texture_modifier = ParticleTextureModifier {
        texture_slot: writer.lit(0u32).expr(),
        sample_mapping: ImageSampleMapping::Modulate,
    };

    // Random rotation
    let orientation = OrientModifier {
        rotation: Some(writer.lit(0.0).uniform(writer.lit(TAU)).expr()),
        ..default()
    };

    let mut module = writer.finish();
    module.add_texture_slot("shape");

    // Set the velocity
    let init_vel = SetAttributeModifier::new(Attribute::VELOCITY, velocity.expr());

    // Spawn from small spherical area at the base
    let init_pos = SetPositionSphereModifier {
        center: module.lit(Vec3::ZERO),
        radius: module.lit(0.4),
        dimension: ShapeDimension::Volume,
    };

    // Short lifetime for fire particles
    let lifetime = module.lit(0.7);
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    // Constant upward acceleration (mimics heat rise)
    let accel = module.lit(Vec3::Y * 0.4);
    let update_accel = AccelModifier::new(accel);

    // Additive blending to simulate light emission
    let alpha_mode = bevy_hanabi::AlphaMode::Add;

    // Size over lifetime modifier: small -> larger -> fade out
    let mut size_curve = Gradient::new();
    size_curve.add_key(0.0, Vec3::splat(0.2)); // start small
    size_curve.add_key(0.3, Vec3::splat(0.5)); // grow
    size_curve.add_key(1.0, Vec3::splat(0.0)); // shrink to nothing

    let size_over_lifetime = SizeOverLifetimeModifier {
        gradient: size_curve,
        screen_space_size: false,
    };

    const MAX_PARTICLES: u32 = 32768;
    let effect = EffectAsset::new(MAX_PARTICLES, SpawnerSettings::rate(150.0.into()), module)
        .with_name("FireEffect")
        .init(init_pos)
        .init(init_vel)
        .init(init_lifetime)
        .with_alpha_mode(alpha_mode)
        .update(update_accel)
        .render(orientation)
        .render(color_over_lifetime)
        .render(particle_texture_modifier)
        .render(size_over_lifetime);

    effects.add(effect)
}
