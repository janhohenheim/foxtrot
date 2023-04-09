use crate::level_instantiation::spawning::despawn::Despawn;
use crate::level_instantiation::spawning::objects::level::Imported;
use anyhow::{Context, Result};
use bevy::pbr::NotShadowCaster;
use bevy::prelude::*;
use bevy_mod_sysfail::macros::*;
use regex::Regex;
use std::sync::LazyLock;

pub(crate) fn set_hidden(mut added_name: Query<(&Name, &mut Visibility), Added<Name>>) {
    #[cfg(feature = "tracing")]
    let _span = info_span!("set_hidden").entered();
    for (name, mut visibility) in added_name.iter_mut() {
        if name.to_lowercase().contains("[hidden]") {
            *visibility = Visibility::Hidden;
        }
    }
}

pub(crate) fn despawn_removed(
    mut commands: Commands,
    mut added_name: Query<(Entity, &Name), Added<Name>>,
) {
    #[cfg(feature = "tracing")]
    let _span = info_span!("despawn_removed").entered();
    for (entity, name) in added_name.iter_mut() {
        if name.to_lowercase().contains("[remove]") {
            commands.entity(entity).insert(Despawn { recursive: true });
        }
    }
}

static COLOR_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\[color:\s*(\d+),\s*(\d+),\s*(\d+),\s*(\d+)\]")
        .expect("Failed to compile color regex")
});

#[sysfail(log(level = "error"))]
pub(crate) fn set_color(
    added_name: Query<(&Name, &Children), Added<Name>>,
    material_handles: Query<&Handle<StandardMaterial>>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
) -> Result<()> {
    #[cfg(feature = "tracing")]
    let _span = info_span!("set_color").entered();
    for (name, children) in added_name.iter() {
        if let Some(captures) = COLOR_REGEX.captures(&name.to_lowercase()) {
            let color = Color::rgba_u8(
                captures[1]
                    .parse()
                    .with_context(|| format!("Failed to parse r component in color: {}", name))?,
                captures[2]
                    .parse()
                    .with_context(|| format!("Failed to parse g component in color: {}", name))?,
                captures[3]
                    .parse()
                    .with_context(|| format!("Failed to parse b component in color: {}", name))?,
                captures[4]
                    .parse()
                    .with_context(|| format!("Failed to parse a component in color: {}", name))?,
            );
            let material_handle = children
                .iter()
                .filter_map(|entity| material_handles.get(*entity).ok())
                .next()
                .with_context(|| {
                    format!(
                        "Failed to find child containing material handle when setting color on: {}",
                        name,
                    )
                })?;
            let material = standard_materials
                .get_mut(material_handle)
                .context("Failed to get standard material from handle")?;
            material.base_color = color;
        }
    }
    Ok(())
}

#[sysfail(log(level = "error"))]
pub(crate) fn set_shadows(
    mut commands: Commands,
    added_mesh: Query<Entity, Added<Handle<Mesh>>>,
    parent_query: Query<&Parent>,
    imported: Query<&Imported>,
    names: Query<&Name>,
) -> Result<()> {
    #[cfg(feature = "tracing")]
    let _span = info_span!("set_shadows").entered();
    for entity in added_mesh.iter() {
        let top_parent = parent_query.iter_ancestors(entity).last();
        let is_imported = top_parent
            .map(|entity| imported.contains(entity))
            .unwrap_or_default();
        if !is_imported {
            continue;
        }
        let parent = parent_query
            .get(entity)
            .context("Failed to get parent of added mesh")?;
        let name = names
            .get(parent.get())
            .context("Failed to get name of parent of added mesh")?;

        if !name.contains("[shadow]") {
            commands.entity(entity).insert(NotShadowCaster);
        }
    }
    Ok(())
}
