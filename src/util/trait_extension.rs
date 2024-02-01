use bevy::prelude::*;
use bevy::render::mesh::{MeshVertexAttributeId, VertexAttributeValues};

pub(crate) trait Vec3Ext: Copy {
    fn is_approx_zero(self) -> bool;
    fn horizontal(self) -> Vec3;
}
impl Vec3Ext for Vec3 {
    #[inline]
    fn is_approx_zero(self) -> bool {
        self.length_squared() < 1e-5
    }

    #[inline]
    fn horizontal(self) -> Vec3 {
        Vec3::new(self.x, 0., self.z)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct SplitVec3 {
    pub(crate) vertical: Vec3,
    pub(crate) horizontal: Vec3,
}

pub(crate) trait Vec2Ext: Copy {
    fn is_approx_zero(self) -> bool;
    fn x0y(self) -> Vec3;
}
impl Vec2Ext for Vec2 {
    #[inline]
    fn is_approx_zero(self) -> bool {
        self.length_squared() < 1e-5
    }

    #[inline]
    fn x0y(self) -> Vec3 {
        Vec3::new(self.x, 0., self.y)
    }
}

pub(crate) trait MeshExt {
    fn transform(&mut self, transform: Transform);
    fn transformed(&self, transform: Transform) -> Mesh;
    fn read_coords_mut(&mut self, id: impl Into<MeshVertexAttributeId>) -> &mut Vec<[f32; 3]>;
    fn find_mesh<'a>(
        parent: Entity,
        children: &'a Query<&Children>,
        meshes: &'a Assets<Mesh>,
        mesh_handles: &'a Query<&Handle<Mesh>>,
    ) -> Option<&'a Mesh>;
}

impl MeshExt for Mesh {
    fn transform(&mut self, transform: Transform) {
        for coords in self.read_coords_mut(Mesh::ATTRIBUTE_POSITION.clone()) {
            let vec3 = (*coords).into();
            let transformed = transform.transform_point(vec3);
            *coords = transformed.into();
        }
        for normal in self.read_coords_mut(Mesh::ATTRIBUTE_NORMAL.clone()) {
            let vec3 = (*normal).into();
            let transformed = transform.rotation.mul_vec3(vec3);
            *normal = transformed.into();
        }
    }

    fn transformed(&self, transform: Transform) -> Mesh {
        let mut mesh = self.clone();
        mesh.transform(transform);
        mesh
    }

    fn read_coords_mut(&mut self, id: impl Into<MeshVertexAttributeId>) -> &mut Vec<[f32; 3]> {
        // Guaranteed by Bevy for the current usage
        match self
            .attribute_mut(id)
            .expect("Failed to read unknown mesh attribute")
        {
            VertexAttributeValues::Float32x3(values) => values,
            // Guaranteed by Bevy for the current usage
            _ => unreachable!(),
        }
    }

    fn find_mesh<'a>(
        parent: Entity,
        children_query: &'a Query<&Children>,
        meshes: &'a Assets<Mesh>,
        mesh_handles: &'a Query<&Handle<Mesh>>,
    ) -> Option<&'a Mesh> {
        if let Ok(children) = children_query.get(parent) {
            for child in children.iter() {
                if let Ok(mesh_handle) = mesh_handles.get(*child) {
                    if let Some(mesh) = meshes.get(mesh_handle) {
                        return Some(mesh);
                    }
                }
            }
        }
        None
    }
}

pub(crate) trait F32Ext: Copy {
    fn is_approx_zero(self) -> bool;
    fn squared(self) -> f32;
    fn lerp(self, other: f32, ratio: f32) -> f32;
}

impl F32Ext for f32 {
    #[inline]
    fn is_approx_zero(self) -> bool {
        self.abs() < 1e-5
    }

    #[inline]
    fn squared(self) -> f32 {
        self * self
    }

    #[inline]
    fn lerp(self, other: f32, ratio: f32) -> f32 {
        self.mul_add(1. - ratio, other * ratio)
    }
}
