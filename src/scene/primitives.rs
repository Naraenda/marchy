use ultraviolet::Vec3;
use crate::engine::marcher::*;
use crate::engine::material::*;

/// A box defined by width, height, and length.
pub struct Cuboid {
    pub material: Material,
    
    pub size: Vec3,
}

impl Marchable for Cuboid {
    fn signed_distance(&self, point: Vec3) -> f32 {
        let q = point.abs() - self.size;
        q.max_by_component(Vec3::zero()).mag() + f32::min(q.component_max(), 0.0)
    }

    fn dist_material(&self, point: Vec3) -> (f32, Material) {
        (self.signed_distance(point), self.material)
    }
}

/// It's a sphere, should be self explanatory.
pub struct Sphere {
    pub material: Material,

    pub radius: f32,
}

impl Marchable for Sphere {
    fn signed_distance(&self, point: Vec3) -> f32 {
        point.mag() - self.radius
    }

    fn dist_material(&self, point: Vec3) -> (f32, Material) {
        (self.signed_distance(point), self.material)
    }
}

/// Donut? yes.
pub struct Torus {
    pub material: Material,

    /// Outer radius
    pub radius_1: f32,

    /// Inner radius
    pub radius_2: f32,

}

impl Marchable for Torus {
    fn signed_distance(&self, point: Vec3) -> f32 {
        let a = (point.x * point.x + point.z * point.z).sqrt() - self.radius_1;
        let b = (a * a + point.y * point.y).sqrt();
        return b - self.radius_2;
    }

    fn dist_material(&self, point: Vec3) -> (f32, Material) {
        (self.signed_distance(point), self.material)
    }
}

pub struct Plane {
    pub material: Material,

    /// Normal of the plane
    pub normal: Vec3,

    /// Perpendicular distance to the origin
    pub height: f32,
}

impl Marchable for Plane {
    fn signed_distance(&self, point: Vec3) -> f32 {
        point.dot(self.normal) + self.height
    }

    fn dist_material(&self, point: Vec3) -> (f32, Material) {
        (self.signed_distance(point), self.material)
    }
}
