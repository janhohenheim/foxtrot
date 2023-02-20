use crate::level_instantiation::spawning::spawn::Despawn;
use bevy::prelude::*;

pub fn set_hidden(mut added_name: Query<(&Name, &mut Visibility), Added<Name>>) {
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
    for event in mesh_asset_events.iter() {
        if let AssetEvent::Created { handle } = event {
            let mesh = meshes
                .get_mut(handle)
                .expect("Failed to get mesh even though it was just created");
            if let Err(e) = mesh.generate_tangents() {
                warn!("Failed to generate tangents for mesh: {}", e);
            }
        }
    }
}
