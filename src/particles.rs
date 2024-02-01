use crate::file_system_interaction::config::GameConfig;
use crate::level_instantiation::spawning::objects::player;
use crate::particles::init::init_effects;
use crate::util::trait_extension::{F32Ext, Vec3Ext};
use crate::GameState;
use anyhow::Result;
use bevy::prelude::*;
use bevy_hanabi::prelude::*;
use bevy_mod_sysfail::sysfail;
use bevy_tnua::prelude::*;
use bevy_xpbd_3d::prelude::*;

mod init;

/// Handles particle effects instantiation and playing.
pub(crate) fn particle_plugin(app: &mut App) {
    app.register_type::<SprintingParticle>()
        .add_plugins(HanabiPlugin)
        .add_systems(OnExit(GameState::Loading), init_effects)
        .add_systems(
            Update,
            play_sprinting_effect.run_if(in_state(GameState::Playing)),
        );
}

#[derive(Debug, Clone, Eq, PartialEq, Component, Reflect, Default)]
#[reflect(Component)]
struct SprintingParticle;

#[sysfail(log(level = "error"))]
fn play_sprinting_effect(
    with_player: Query<(&Transform, &TnuaController, &LinearVelocity), Without<SprintingParticle>>,
    mut with_particle: Query<(&mut Transform, &mut EffectSpawner), With<SprintingParticle>>,
    config: Res<GameConfig>,
) -> Result<()> {
    for (player_transform, controller, velocity) in with_player.iter() {
        let horizontal_speed_squared = velocity
            .split(player_transform.up())
            .horizontal
            .length_squared();
        for (mut particle_transform, mut effect_spawner) in with_particle.iter_mut() {
            let threshold = config.player.sprint_effect_speed_threshold;
            if !controller.is_airborne().unwrap_or_default()
                && horizontal_speed_squared > threshold.squared()
            {
                let translation = player_transform.translation
                    - player_transform.up() * (player::HEIGHT / 2. + player::RADIUS);
                *particle_transform = player_transform.with_translation(translation);
                effect_spawner.set_active(true);
            } else {
                effect_spawner.set_active(false);
            }
        }
    }
    Ok(())
}
