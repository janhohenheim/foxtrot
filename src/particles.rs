use crate::file_system_interaction::config::GameConfig;
use crate::player_control::player_embodiment::Player;
use crate::util::trait_extension::{F32Ext, Vec3Ext};
use crate::GameState;
use anyhow::Result;
use bevy::prelude::*;
use bevy_hanabi::prelude::*;
use bevy_mod_sysfail::sysfail;
use bevy_tnua::prelude::*;
use bevy_xpbd_3d::PhysicsSet;
pub(crate) use creation::*;

mod creation;

/// Handles particle effects instantiation and playing.
pub(crate) fn particle_plugin(app: &mut App) {
    app.register_type::<SprintingParticle>()
        .add_plugins(HanabiPlugin)
        .add_systems(
            Update,
            play_sprinting_effect
                .run_if(in_state(GameState::Playing))
                .after(PhysicsSet::Sync),
        );
}

#[derive(Debug, Clone, Eq, PartialEq, Component, Reflect, Default)]
#[reflect(Component)]
struct SprintingParticle;

#[sysfail(log(level = "error"))]
fn play_sprinting_effect(
    with_player: Query<&TnuaController, With<Player>>,
    mut with_particle: Query<&mut EffectSpawner, With<SprintingParticle>>,
    config: Res<GameConfig>,
) -> Result<()> {
    for controller in with_player.iter() {
        let Some((_, basis_state)) = controller.concrete_basis::<TnuaBuiltinWalk>() else {
            continue;
        };
        let horizontal_speed_squared = basis_state.running_velocity.horizontal().length_squared();
        for mut effect_spawner in with_particle.iter_mut() {
            let threshold = config.player.sprint_effect_speed_threshold;
            let active = !controller.is_airborne().unwrap_or_default()
                && horizontal_speed_squared > threshold.squared();
            effect_spawner.set_active(active);
        }
    }
    Ok(())
}
