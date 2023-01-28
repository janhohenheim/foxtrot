use bevy::prelude::*;
use bevy::render::mesh::{MeshVertexAttributeId, PrimitiveTopology, VertexAttributeValues};

pub trait Vec3Ext {
    #[allow(clippy::wrong_self_convention)] // Because [`Vec3`] is [`Copy`]
    fn is_approx_zero(self) -> bool;
    fn x0z(self) -> Vec3;
}
impl Vec3Ext for Vec3 {
    fn is_approx_zero(self) -> bool {
        [self.x, self.y, self.z].iter().all(|&x| x.abs() < 1e-5)
    }
    fn x0z(self) -> Vec3 {
        Vec3::new(self.x, 0., self.z)
    }
}

pub trait MeshExt {
    fn transform(&mut self, transform: Transform);
    fn transformed(&self, transform: Transform) -> Mesh;
    fn read_coords_mut(&mut self, id: impl Into<MeshVertexAttributeId>) -> &mut Vec<[f32; 3]>;
    fn search_in_children<'a>(
        children: &'a Children,
        meshes: &'a Assets<Mesh>,
        mesh_handles: &'a Query<&Handle<Mesh>>,
    ) -> (Entity, &'a Mesh);
}

impl MeshExt for Mesh {
    fn transform(&mut self, transform: Transform) {
        for attribute in [Mesh::ATTRIBUTE_POSITION, Mesh::ATTRIBUTE_NORMAL] {
            for coords in self.read_coords_mut(attribute.clone()) {
                let vec3 = (*coords).into();
                let transformed = transform.transform_point(vec3);
                *coords = transformed.into();
            }
        }
    }

    fn transformed(&self, transform: Transform) -> Mesh {
        let mut mesh = self.clone();
        mesh.transform(transform);
        mesh
    }

    fn read_coords_mut(&mut self, id: impl Into<MeshVertexAttributeId>) -> &mut Vec<[f32; 3]> {
        match self.attribute_mut(id).unwrap() {
            VertexAttributeValues::Float32x3(values) => values,
            // Guaranteed by Bevy
            _ => unreachable!(),
        }
    }

    fn search_in_children<'a>(
        children: &'a Children,
        meshes: &'a Assets<Mesh>,
        mesh_handles: &'a Query<&Handle<Mesh>>,
    ) -> (Entity, &'a Mesh) {
        let entity_handles: Vec<_> = children
            .iter()
            .filter_map(|entity| mesh_handles.get(*entity).ok().map(|mesh| (*entity, mesh)))
            .collect();
        assert_eq!(
            entity_handles.len(),
            1,
            "Collider must contain exactly one mesh, but found {}",
            entity_handles.len()
        );
        let (entity, mesh_handle) = entity_handles.first().unwrap();
        let mesh = meshes.get(mesh_handle).unwrap();
        assert_eq!(mesh.primitive_topology(), PrimitiveTopology::TriangleList);
        (*entity, mesh)
    }
}
