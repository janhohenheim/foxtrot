use crate::level_instantiation::spawning::spawn::Despawn;
use anyhow::{Context, Result};
use bevy::prelude::*;
use regex::Regex;
use std::sync::LazyLock;

pub fn set_hidden(mut added_name: Query<(&Name, &mut Visibility), Added<Name>>) {
    #[cfg(feature = "tracing")]
    let _span = info_span!("set_hidden").entered();
    for (name, mut visibility) in added_name.iter_mut() {
        if name.to_lowercase().contains("[hidden]") {
            visibility.is_visible = false;
        }
    }
}

pub fn despawn_removed(
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

/// Needed for normal mapping,
/// see [`StandardMaterial::normal_map_texture`](https://docs.rs/bevy/latest/bevy/pbr/struct.StandardMaterial.html#structfield.normal_map_texture).
pub fn generate_tangents(
    mut mesh_asset_events: EventReader<AssetEvent<Mesh>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    #[cfg(feature = "tracing")]
    let _span = info_span!("generate_tangents").entered();
    for event in mesh_asset_events.iter() {
        if let AssetEvent::Created { handle } = event {
            // Guaranteed to work because we just created the mesh
            let mesh = meshes
                .get_mut(handle)
                .expect("Failed to get mesh even though it was just created");
            if let Err(e) = mesh.generate_tangents() {
                warn!("Failed to generate tangents for mesh: {}", e);
            }
        }
    }
}

static COLOR_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\[color:\s*(\d+),\s*(\d+),\s*(\d+),\s*(\d+)\]")
        .expect("Failed to compile color regex")
});

pub fn set_color(
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
