use crate::level_instantiation::spawning::animation_link::link_animations;
use crate::level_instantiation::spawning::objects::*;
use crate::GameState;
pub(crate) use animation_link::AnimationEntityLink;
use bevy::gltf::GltfExtras;
use bevy::prelude::*;
use serde_json::{Result, Value};
mod animation_link;
pub(crate) mod objects;

pub(crate) fn spawning_plugin(app: &mut App) {
    app.register_type::<AnimationEntityLink>()
        .register_type::<camera::IngameCameraMarker>()
        .register_type::<orb::Orb>()
        .register_type::<sunlight::Sun>()
        .register_type::<Hidden>()
        .add_systems(
            Update,
            (
                add_components_from_gltf_extras,
                camera::spawn,
                orb::spawn,
                player::spawn,
                npc::spawn,
                sunlight::spawn,
                //link_animations,
                hide,
            )
                .run_if(in_state(GameState::Playing)),
        );
}

// Reads the extras filed from the GLTF. In Blender, this is the "Custom Attributes" you can set on an object.
// We treat each extra as a indication that we want to inject a marker struct for populating the object later.
fn add_components_from_gltf_extras(extras: Query<(Entity, &GltfExtras), Added<GltfExtras>>) {
    for (entity, extra) in extras.iter() {
        let Ok(json) = serde_json::from_str(extra.value) else {
            continue;
        };
        let components = entity.keys();
    }
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Reflect, Component)]
#[reflect(Component)]
pub(crate) struct Hidden;

fn hide(hidden: Query<Entity, Added<Hidden>>, mut commands: Commands) {
    for entity in hidden.iter() {
        commands.entity(entity).insert(Visibility::Hidden);
    }
}
