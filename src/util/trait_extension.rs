use bevy::prelude::*;

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

pub(crate) trait Vec2Ext: Copy {
    fn is_approx_zero(self) -> bool;
}
impl Vec2Ext for Vec2 {
    #[inline]
    fn is_approx_zero(self) -> bool {
        self.length_squared() < 1e-5
    }
}

pub(crate) trait F32Ext: Copy {
    fn squared(self) -> f32;
}

impl F32Ext for f32 {
    #[inline]
    fn squared(self) -> f32 {
        self * self
    }
}
