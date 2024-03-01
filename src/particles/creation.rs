use crate::{level_instantiation::on_spawn::player, particles::SprintingParticle};
use bevy::{pbr::NotShadowReceiver, prelude::*};
use bevy_hanabi::prelude::*;

pub(crate) fn create_sprint_particle_bundle(effects: &mut Assets<EffectAsset>) -> impl Bundle {
    let sprinting = create_sprinting_effect(effects);
    (
        Name::new("Sprinting particle"),
        SprintingParticle,
        ParticleEffectBundle {
            effect: sprinting,
            ..default()
        },
        NotShadowReceiver,
    )
}

fn create_sprinting_effect(effects: &mut Assets<EffectAsset>) -> ParticleEffect {
    let mut color_gradient = Gradient::new();
    color_gradient.add_key(0.0, Vec4::new(1.2, 1.0, 1.0, 0.6));
    color_gradient.add_key(0.1, Vec4::new(1.2, 1.0, 1.0, 0.4));
    color_gradient.add_key(0.6, Vec4::new(1.2, 1.0, 1.0, 0.2));
    color_gradient.add_key(1.0, Vec4::new(1.2, 1.0, 1.0, 0.0));

    let mut size_gradient = Gradient::new();
    size_gradient.add_key(0.0, Vec2::splat(0.1));
    size_gradient.add_key(0.3, Vec2::splat(0.12));
    size_gradient.add_key(0.6, Vec2::splat(0.15));
    size_gradient.add_key(1.0, Vec2::splat(0.2));

    let mut module = Module::default();
    let position_circle_modifier = SetPositionCircleModifier {
        dimension: ShapeDimension::Volume,
        radius: module.lit(player::RADIUS * 0.5),
        center: module.lit(Vec3::ZERO),
        axis: module.lit(Vec3::Y),
    };
    let velocity_sphere_modifier = SetVelocitySphereModifier {
        speed: module.lit(1.0),
        center: module.lit(Vec3::ZERO),
    };
    let lifetime = SetAttributeModifier::new(Attribute::LIFETIME, module.lit(0.8));
    let linear_drag_modifier = LinearDragModifier {
        drag: module.lit(5.0),
    };
    let orient_modifier = OrientModifier {
        mode: OrientMode::FaceCameraPosition,
        rotation: None,
    };
    let accel_modifier = AccelModifier::new(module.lit(Vec3::new(0., 1., 0.)));

    ParticleEffect::new(
        effects.add(
            EffectAsset::new(
                100,
                Spawner::rate(10.0.into()).with_starts_active(false),
                module,
            )
            .with_name("Sprint")
            .init(position_circle_modifier)
            .init(velocity_sphere_modifier)
            .init(lifetime)
            .update(linear_drag_modifier)
            .render(orient_modifier)
            .update(accel_modifier)
            .render(ColorOverLifetimeModifier {
                gradient: color_gradient,
            })
            .render(SizeOverLifetimeModifier {
                gradient: size_gradient,
                screen_space_size: false,
            }),
        ),
    )
}
