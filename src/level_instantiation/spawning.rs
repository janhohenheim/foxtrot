use crate::level_instantiation::spawning::animation_link::link_animations;
use crate::level_instantiation::spawning::objects::*;
use crate::GameState;
pub(crate) use animation_link::AnimationEntityLink;
use anyhow::Result;
use bevy::gltf::GltfExtras;
use bevy::prelude::*;
use bevy::reflect::serde::TypedReflectDeserializer;
use bevy_mod_sysfail::sysfail;
use serde::de::DeserializeSeed;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::ops::Deref;

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
// See this as a simplified version of https://github.com/kaosat-dev/Blender_bevy_components_workflow/tree/main/crates/bevy_gltf_components
#[sysfail(log(level = "error"))]
fn add_components_from_gltf_extras(
    mut commands: Commands,
    extras: Query<(Entity, &GltfExtras), Added<GltfExtras>>,
    type_registry: Res<AppTypeRegistry>,
) -> Result<()> {
    for (entity, extra) in extras.iter() {
        let Ok(json) = serde_json::from_str::<Value>(&extra.value) else {
            continue;
        };
        let Some(object) = json.as_object() else {
            continue;
        };
        let type_registry = type_registry.read();
        for component_name in object.keys() {
            let Some(type_registration) = type_registry.get_with_short_type_path(component_name)
            else {
                warn!("No type or ambiguous registration found for component {component_name}");
                continue;
            };
            let reflection_deserializer =
                TypedReflectDeserializer::new(&type_registration, type_registry.deref());
            let mut ron_deserializer = ron::Deserializer::from_str(&component_name)?;
            let component = reflection_deserializer.deserialize(&mut ron_deserializer);

            if let Ok(component) = component {
                info!("Deserialized {component_name}");
            } else {
                warn!("Failed to deserialize component {component_name}");
            }
        }
    }
    Ok(())
}

#[derive(Debug, Clone, Eq, PartialEq, Component, Reflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub(crate) struct Hidden;

fn hide(hidden: Query<Entity, Added<Hidden>>, mut commands: Commands) {
    for entity in hidden.iter() {
        commands.entity(entity).insert(Visibility::Hidden);
    }
}
