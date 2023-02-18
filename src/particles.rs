use crate::level_instantiation::spawning::objects::player;
use crate::movement::general_movement::Grounded;
use crate::particles::init::init_effects;
use crate::util::trait_extension::{F32Ext, Vec3Ext};
use crate::GameState;
use bevy::prelude::*;
use bevy_hanabi::prelude::*;
use bevy_rapier3d::prelude::*;

mod init;

/// Handles particle effects instantiation and playing.
pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<SprintingParticle>()
            .add_plugin(HanabiPlugin)
            .add_system_set(SystemSet::on_exit(GameState::Loading).with_system(init_effects))
            .add_system_set(
                SystemSet::on_update(GameState::Playing).with_system(play_sprinting_effect),
            );
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Component, Reflect, Default)]
#[reflect(Component)]
struct SprintingParticle;

fn play_sprinting_effect(
    with_player: Query<(&Transform, &Grounded, &Velocity), Without<SprintingParticle>>,
    mut with_particle: Query<(&mut Transform, &mut ParticleEffect), With<SprintingParticle>>,
) {
    const SPRINT_EFFECT_SPEED_THRESHOLD: f32 = 7.;
    for (player_transform, grounded, velocity) in with_player.iter() {
        let horizontal_speed_squared = velocity
            .linvel
            .split(player_transform.up())
            .horizontal
            .length_squared();
        for (mut particle_transform, mut effect) in with_particle.iter_mut() {
            if grounded.0 && horizontal_speed_squared > SPRINT_EFFECT_SPEED_THRESHOLD.squared() {
                let translation = player_transform.translation
                    - player_transform.up() * (player::HEIGHT / 2. + player::RADIUS);
                *particle_transform = player_transform.with_translation(translation);
                effect.maybe_spawner().unwrap().set_active(true);
            } else {
                effect.maybe_spawner().unwrap().set_active(false);
            }
        }
    }
}
