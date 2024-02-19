use crate::{level_instantiation::spawning::objects::*, GameState};
use bevy::prelude::*;
use bevy_gltf_components::ComponentsFromGltfPlugin;
use bevy_xpbd_3d::PhysicsSet;
use serde::{Deserialize, Serialize};

pub(crate) mod objects;

pub(crate) fn spawning_plugin(app: &mut App) {
    app.add_plugins(ComponentsFromGltfPlugin::default())
        .register_type::<camera::IngameCameraMarker>()
        .register_type::<orb::Orb>()
        .register_type::<sunlight::Sun>()
        .register_type::<Hidden>()
        .register_type::<ground::Grass>()
        .add_systems(
            Update,
            (
                ground::spawn,
                camera::spawn,
                orb::spawn,
                player::spawn,
                npc::spawn,
                sunlight::spawn,
                hide.after(PhysicsSet::Sync),
            )
                .run_if(in_state(GameState::Playing)),
        );
}

#[derive(Debug, Clone, Eq, PartialEq, Component, Reflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub(crate) struct Hidden;

fn hide(hidden: Query<Entity, Added<Hidden>>, mut commands: Commands) {
    for entity in hidden.iter() {
        commands.entity(entity).insert(Visibility::Hidden);
    }
}
