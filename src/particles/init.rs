use crate::level_instantiation::spawning::objects::player;
use crate::particles::SprintingParticle;
use bevy::pbr::NotShadowReceiver;
use bevy::prelude::*;
use bevy_hanabi::prelude::*;

pub(crate) fn init_effects(mut commands: Commands, mut effects: ResMut<Assets<EffectAsset>>) {
    let sprinting = create_sprinting_effect(&mut effects);
    commands.spawn((
        Name::new("Sprinting particle"),
        SprintingParticle,
        ParticleEffectBundle {
            effect: sprinting,
            ..default()
        },
        NotShadowReceiver,
    ));
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

    ParticleEffect::new(
        effects.add(
            EffectAsset {
                name: "Sprint".to_string(),
                capacity: 100,
                spawner: Spawner::rate(10.0.into()).with_active(false),
                ..Default::default()
            }
            .init(InitPositionCircleModifier {
                dimension: ShapeDimension::Volume,
                radius: player::RADIUS * 0.5,
                center: Vec3::ZERO,
                axis: Vec3::Y,
            })
            .init(InitVelocitySphereModifier {
                speed: 1_f32.into(),
                center: Vec3::ZERO,
            })
            .init(InitLifetimeModifier {
                lifetime: 0.8.into(),
            })
            .update(LinearDragModifier { drag: 5. })
            .render(BillboardModifier {})
            .update(AccelModifier::constant(Vec3::new(0., 1., 0.)))
            .render(ColorOverLifetimeModifier {
                gradient: color_gradient,
            })
            .render(SizeOverLifetimeModifier {
                gradient: size_gradient,
            }),
        ),
    )
}
