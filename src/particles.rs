use crate::{
    file_system_interaction::config::GameConfig,
    level_instantiation::on_spawn::Player,
    util::math_trait_ext::{F32Ext, Vec3Ext},
    GameState,
};
use bevy::prelude::*;
use bevy_hanabi::prelude::*;
use bevy_mod_sysfail::prelude::*;
use bevy_tnua::prelude::*;
use bevy_xpbd_3d::PhysicsSet;
pub(crate) use creation::*;

mod creation;

/// Handles particle effects instantiation and playing.
pub(super) fn plugin(app: &mut App) {
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

#[sysfail(Log<anyhow::Error, Error>)]
fn play_sprinting_effect(
    with_player: Query<&TnuaController, With<Player>>,
    mut with_particle: Query<&mut EffectSpawner, With<SprintingParticle>>,
    config: Res<GameConfig>,
) {
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
}
