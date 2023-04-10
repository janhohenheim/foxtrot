use crate::file_system_interaction::config::GameConfig;
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
pub(crate) fn particle_plugin(app: &mut App) {
    app.register_type::<SprintingParticle>()
        .add_plugin(HanabiPlugin)
        .add_system(init_effects.in_schedule(OnExit(GameState::Loading)))
        .add_system(play_sprinting_effect.in_set(OnUpdate(GameState::Playing)));
}

#[derive(Debug, Clone, Eq, PartialEq, Component, Reflect, Default)]
#[reflect(Component)]
struct SprintingParticle;

fn play_sprinting_effect(
    with_player: Query<(&Transform, &Grounded, &Velocity), Without<SprintingParticle>>,
    mut with_particle: Query<(&mut Transform, &mut ParticleEffect), With<SprintingParticle>>,
    config: Res<GameConfig>,
) {
    for (player_transform, grounded, velocity) in with_player.iter() {
        let horizontal_speed_squared = velocity
            .linvel
            .split(player_transform.up())
            .horizontal
            .length_squared();
        for (mut particle_transform, mut effect) in with_particle.iter_mut() {
            let threshold = config.player.sprint_effect_speed_threshold;
            if grounded.0 && horizontal_speed_squared > threshold.squared() {
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
