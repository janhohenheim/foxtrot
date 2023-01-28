use bevy::prelude::*;

pub trait Vec3Ext {
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
