use crate::util::trait_extension::MeshExt;
use anyhow::{bail, Context, Result};
use bevy::prelude::*;
use bevy::render::mesh::{PrimitiveTopology, VertexAttributeValues};
use bevy::transform::TransformSystem;
use bevy_mod_sysfail::macros::*;
use rand::{rngs::SmallRng, Rng, SeedableRng};
use warbler_grass::prelude::*;

pub(crate) fn grass_plugin(app: &mut App) {
    app.add_plugin(WarblersPlugin).add_system(
        add_grass
            .after(TransformSystem::TransformPropagate)
            .in_base_set(CoreSet::PostUpdate),
    );
}

#[sysfail(log(level = "error"))]
pub(crate) fn add_grass(
    mut commands: Commands,
    added_name: Query<(Entity, &Name), Added<Name>>,
    meshes: Res<Assets<Mesh>>,
    children_query: Query<&Children>,
    mesh_handles: Query<&Handle<Mesh>>,
    global_transforms: Query<&GlobalTransform>,
) -> Result<()> {
    for (parent_entity, name) in added_name.iter() {
        if name.contains("[grass]") {
            for (child_entity, mesh) in
                Mesh::search_in_children(parent_entity, &children_query, &meshes, &mesh_handles)
            {
                let transform = global_transforms
                    .get(child_entity)
                    .context("Could not get global transform of grass mesh")?
                    .compute_transform();
                let triangles = read_triangles(mesh)?;
                let triangles = triangles
                    .map(|triangle| triangle.map(|position| transform.transform_point(position)));

                let rng = SmallRng::from_entropy();
                const BLADES_PER_SQUARE_METER: f32 = 10.0;
                let positions = triangles
                    .flat_map(|triangle| {
                        let area = area_of_triangle(&triangle);
                        let blade_count = (area * BLADES_PER_SQUARE_METER) as usize;
                        let mut rng = rng.clone();
                        (0..blade_count).map(move |_| {
                            pick_uniform_random_point_in_triangle(&mut rng, &triangle)
                        })
                    })
                    .collect();
                let height = 0.7;
                commands.spawn((
                    Name::new("Grass"),
                    WarblersExplicitBundle {
                        grass: Grass { positions, height },
                        ..default()
                    },
                ));
            }
        }
    }
    Ok(())
}

fn read_triangles(mesh: &Mesh) -> Result<impl Iterator<Item = [Vec3; 3]> + '_> {
    if mesh.primitive_topology() != PrimitiveTopology::TriangleList {
        bail!("Grass mesh must be a triangle list");
    }
    let positions = mesh
        .attribute(Mesh::ATTRIBUTE_POSITION)
        .context("Failed to get mesh position attribute when building grass")?;
    let positions = match positions {
        VertexAttributeValues::Float32x3(positions) => positions,
        _ => bail!("Grass mesh position attribute must be a float32x3"),
    };
    let indices = mesh
        .indices()
        .context("Failed to get mesh indices when building grass")?;
    let triangle_vertex_indices = indices.iter().array_chunks::<3>();
    let triangles = triangle_vertex_indices.map(|indices| {
        indices.map(|index| {
            let position = positions[index];
            Vec3::from(position)
        })
    });
    Ok(triangles)
}

/// Source: <https://math.stackexchange.com/questions/18686/uniform-random-point-in-triangle-in-3d>
fn pick_uniform_random_point_in_triangle(rng: &mut SmallRng, &[a, b, c]: &[Vec3; 3]) -> Vec3 {
    let r1: f32 = rng.gen();
    let root_r1 = r1.sqrt();
    let r2: f32 = rng.gen();
    (1.0 - root_r1) * a + (root_r1 * (1.0 - r2)) * b + (root_r1 * r2) * c
}

/// Source: <https://math.stackexchange.com/a/128999/419398>
fn area_of_triangle(&[a, b, c]: &[Vec3; 3]) -> f32 {
    let ab = b - a;
    let ac = c - a;
    let cross = ab.cross(ac);
    cross.length() / 2.0
}
