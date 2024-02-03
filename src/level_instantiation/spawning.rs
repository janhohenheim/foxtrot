use crate::level_instantiation::spawning::objects::*;
use crate::GameState;
use anyhow::{Context, Result};
use bevy::gltf::GltfExtras;
use bevy::prelude::*;
use bevy::reflect::serde::TypedReflectDeserializer;
use bevy::utils::HashMap;

use bevy_xpbd_3d::PhysicsSet;
use serde::de::DeserializeSeed;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::ops::Deref;

pub(crate) mod objects;

pub(crate) fn spawning_plugin(app: &mut App) {
    app.register_type::<camera::IngameCameraMarker>()
        .register_type::<orb::Orb>()
        .register_type::<sunlight::Sun>()
        .register_type::<grass::Grass>()
        .register_type::<Hidden>()
        .add_systems(Update, add_components_from_gltf_extras.map(Result::unwrap))
        .add_systems(
            Update,
            (
                camera::spawn,
                orb::spawn,
                player::spawn,
                npc::spawn,
                sunlight::spawn,
                grass::spawn,
                hide.after(PhysicsSet::Sync),
            )
                .run_if(in_state(GameState::Playing)),
        );
}

// Reads the extras filed from the GLTF. In Blender, this is the "Custom Attributes" you can set on an object.
// We treat each extra as a indication that we want to inject a marker struct for populating the object later.
// See this as a simplified version of https://github.com/kaosat-dev/Blender_bevy_components_workflow/tree/main/crates/bevy_gltf_components

fn add_components_from_gltf_extras(world: &mut World) -> Result<()> {
    let mut extras = world.query::<(Entity, &GltfExtras)>();
    let mut components = HashMap::new();
    for (entity, extra) in extras.iter(world) {
        let Ok(json) = serde_json::from_str::<Value>(&extra.value) else {
            continue;
        };
        let Some(object) = json.as_object() else {
            continue;
        };
        let component_names: Vec<_> = object.keys().map(|k| k.to_string()).collect();
        components.insert(entity, component_names);
    }
    for (entity, component_names) in components {
        for component_name in component_names {
            let (type_registration, component, component_name) = {
                let type_registry: &AppTypeRegistry = world.resource();
                let type_registry = type_registry.read();
                let Some(type_registration) =
                    type_registry.get_with_short_type_path(&component_name)
                else {
                    warn!("No type or ambiguous registration found for component {component_name}");
                    continue;
                };
                let reflection_deserializer =
                    TypedReflectDeserializer::new(type_registration, type_registry.deref());
                let mut ron_deserializer = ron::Deserializer::from_str(&component_name)?;
                let component = reflection_deserializer.deserialize(&mut ron_deserializer)?;
                (type_registration.clone(), component, component_name)
            };

            let mut entity_mut = world.entity_mut(entity);
            type_registration
                .data::<ReflectComponent>()
                .with_context(|| format!("{component_name} does not reflect Component"))?
                .insert(&mut entity_mut, &*component);
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
