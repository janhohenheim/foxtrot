use crate::file_system_interaction::level_serialization::{CurrentLevel, WorldLoadRequest};
use crate::level_instantiation::spawning::{DelayedSpawnEvent, GameObject, SpawnEvent};
use crate::shader::Materials;
use crate::GameState;
use bevy::pbr::NotShadowCaster;
use bevy::prelude::*;
use bevy::render::mesh::VertexAttributeValues;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup));
    }
}

fn setup(
    mut commands: Commands,
    mut loader: EventWriter<WorldLoadRequest>,
    mut delayed_spawner: EventWriter<DelayedSpawnEvent>,
    current_level: Option<Res<CurrentLevel>>,
    mut meshes: ResMut<Assets<Mesh>>,
    materials: Res<Materials>,
) {
    if current_level.is_some() {
        return;
    }

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.3,
    });

    loader.send(WorldLoadRequest {
        filename: "old_town".to_string(),
    });

    // Make sure the player is spawned after the level
    delayed_spawner.send(DelayedSpawnEvent {
        tick_delay: 2,
        event: SpawnEvent {
            object: GameObject::Player,
            transform: Transform::from_xyz(0., 1.5, 0.),
        },
    });
    let mut mesh = Mesh::from(shape::UVSphere {
        radius: 100.,
        ..default()
    });
    let normals = match mesh.attribute_mut(Mesh::ATTRIBUTE_NORMAL).unwrap() {
        VertexAttributeValues::Float32x3(values) => values,
        _ => unreachable!(),
    };
    for normal in normals.iter_mut() {
        *normal = normal.map(|n| -n);
    }
    commands.spawn((
        Name::new("Skydome"),
        NotShadowCaster,
        MaterialMeshBundle {
            mesh: meshes.add(mesh),
            material: materials.skydome.clone(),
            ..default()
        },
    ));
}
