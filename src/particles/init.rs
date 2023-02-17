use crate::particles::SprintingParticle;
use bevy::prelude::*;
use bevy_hanabi::prelude::*;

pub fn init_effects(mut commands: Commands, mut effects: ResMut<Assets<EffectAsset>>) {
    let sprinting = create_sprinting_effect(&mut effects);
    commands.spawn((
        Name::new("Sprinting particle"),
        SprintingParticle,
        ParticleEffectBundle {
            effect: sprinting,
            ..default()
        },
    ));
}

fn create_sprinting_effect(effects: &mut Assets<EffectAsset>) -> ParticleEffect {
    let mut color_gradient1 = Gradient::new();
    color_gradient1.add_key(0.0, Vec4::new(4.0, 4.0, 4.0, 1.0));
    color_gradient1.add_key(0.1, Vec4::new(4.0, 4.0, 0.0, 1.0));
    color_gradient1.add_key(0.9, Vec4::new(4.0, 0.0, 0.0, 1.0));
    color_gradient1.add_key(1.0, Vec4::new(4.0, 0.0, 0.0, 0.0));

    let mut size_gradient1 = Gradient::new();
    size_gradient1.add_key(0.0, Vec2::splat(0.1));
    size_gradient1.add_key(0.3, Vec2::splat(0.1));
    size_gradient1.add_key(1.0, Vec2::splat(0.0));

    ParticleEffect::new(
        effects.add(
            EffectAsset {
                name: "firework".to_string(),
                capacity: 100,
                spawner: Spawner::burst(500.0.into(), 0.5.into()).with_active(false),
                ..Default::default()
            }
            .init(PositionSphereModifier {
                dimension: ShapeDimension::Volume,
                radius: 2.,
                speed: 70_f32.into(),
                center: Vec3::ZERO,
            })
            .init(ParticleLifetimeModifier { lifetime: 1. })
            .update(LinearDragModifier { drag: 5. })
            .update(AccelModifier {
                accel: Vec3::new(0., -8., 0.),
            })
            .render(ColorOverLifetimeModifier {
                gradient: color_gradient1,
            })
            .render(SizeOverLifetimeModifier {
                gradient: size_gradient1,
            }),
        ),
    )
}
