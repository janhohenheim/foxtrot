use bevy::prelude::*;
use bevy::render::mesh::{MeshVertexAttributeId, PrimitiveTopology, VertexAttributeValues};

pub(crate) trait Vec3Ext: Copy {
    fn is_approx_zero(self) -> bool;
    fn split(self, up: Vec3) -> SplitVec3;
}
impl Vec3Ext for Vec3 {
    #[inline]
    fn is_approx_zero(self) -> bool {
        self.length_squared() < 1e-5
    }

    #[inline]
    fn split(self, up: Vec3) -> SplitVec3 {
        let vertical = up * self.dot(up);
        let horizontal = self - vertical;
        SplitVec3 {
            vertical,
            horizontal,
        }
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
    fn search_in_children<'a>(
        parent: Entity,
        children: &'a Query<&Children>,
        meshes: &'a Assets<Mesh>,
        mesh_handles: &'a Query<&Handle<Mesh>>,
    ) -> Vec<(Entity, &'a Mesh)>;
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

    fn search_in_children<'a>(
        parent: Entity,
        children_query: &'a Query<&Children>,
        meshes: &'a Assets<Mesh>,
        mesh_handles: &'a Query<&Handle<Mesh>>,
    ) -> Vec<(Entity, &'a Mesh)> {
        if let Ok(children) = children_query.get(parent) {
            let mut result: Vec<_> = children
                .iter()
                .filter_map(|entity| mesh_handles.get(*entity).ok().map(|mesh| (*entity, mesh)))
                .map(|(entity, mesh_handle)| {
                    (
                        entity,
                        meshes
                            .get(mesh_handle)
                            .expect("Failed to get mesh from handle"),
                    )
                })
                .map(|(entity, mesh)| {
                    assert_eq!(mesh.primitive_topology(), PrimitiveTopology::TriangleList);
                    (entity, mesh)
                })
                .collect();
            let mut inner_result = children
                .iter()
                .flat_map(|entity| {
                    Self::search_in_children(*entity, children_query, meshes, mesh_handles)
                })
                .collect();
            result.append(&mut inner_result);
            result
        } else {
            Vec::new()
        }
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

pub(crate) trait TransformExt: Copy {
    fn horizontally_looking_at(self, target: Vec3, up: Vec3) -> Transform;
    fn lerp(self, other: Transform, ratio: f32) -> Transform;
}

impl TransformExt for Transform {
    fn horizontally_looking_at(self, target: Vec3, up: Vec3) -> Transform {
        let direction = target - self.translation;
        let horizontal_direction = direction - up * direction.dot(up);
        let look_target = self.translation + horizontal_direction;
        self.looking_at(look_target, up)
    }

    fn lerp(self, other: Transform, ratio: f32) -> Transform {
        let translation = self.translation.lerp(other.translation, ratio);
        let rotation = self.rotation.slerp(other.rotation, ratio);
        let scale = self.scale.lerp(other.scale, ratio);
        Transform {
            translation,
            rotation,
            scale,
        }
    }
}
