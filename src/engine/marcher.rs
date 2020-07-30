use crate::engine::*;
use crate::utils::Mixable;

pub const EPSILON        : f32 = 0.001;
pub const MAX_DISTANCE   : f32 = 256.0;
pub const MAX_ITERATIONS : u32 = 64;
pub const MAX_BOUNCES    : u32 = 4;

/// A piece of geometry that can be shaded
pub struct Fragment {
    pub position: Vec3,
    pub normal  : Vec3,
    pub material: Material,
}

pub trait Marchable: 'static {
    /// Returns the signed distances from a point. Negative means we are inside something.
    fn signed_distance(&self, point: Vec3) -> f32;

    /// Same as `signed_distance` but also returns the material of the nearest surface.
    fn dist_material(&self, point: Vec3) -> (f32, Material);
}

impl dyn Marchable {
    /// Normal vector from a point near the surface described by the signed distance function.
    pub fn normal(&self, point: Vec3) -> Vec3 {
        let distance_x: f32 = self.signed_distance(point + Vec3::new(EPSILON, 0.0, 0.0));
        let distance_y: f32 = self.signed_distance(point + Vec3::new(0.0, EPSILON, 0.0));
        let distance_z: f32 = self.signed_distance(point + Vec3::new(0.0, 0.0, EPSILON));
        ((Vec3::new(distance_x, distance_y, distance_z) - point) / EPSILON).normalized()
    }

    /// Ray marches from `origin` towards `direction` using a maximum of `MAX_ITERATIONS` steps.
    pub fn ray_march(&self, origin: Vec3, direction: Vec3, lights: &Vec<Box<dyn Light>>, depth: u32) -> Vec3 {
        // limit the amount of light bounces
        if depth > MAX_BOUNCES {
            return Vec3::zero();
        }

        let mut position = origin;
        for _ in 0..MAX_ITERATIONS {
            let (distance, material) = self.dist_material(position);
            if distance < EPSILON {
                let normal = self.normal(position);
                let fragment = Fragment {
                    position: position,
                    normal  : normal,
                    material: material,
                };

                // Result color
                let mut result = Vec3::zero();

                // Shading
                for light in lights {
                    result += light.luminate(&fragment, self);
                }

                // Reflections
                if material.reflective > 0.0 {
                    let reflection_direction = (-direction).reflected(normal);
                    let reflection = self.ray_march(position + reflection_direction * EPSILON * 2.0, reflection_direction, lights, depth + 1);
                    result = Mixable::mix(result, reflection, material.reflective);
                }

                return result;
            }
            if distance > MAX_DISTANCE {
                return Vec3::zero();
            }
            position = position + direction * distance;
        }

        // Could not reach anything :(
        return Vec3::zero();
    }

    /// Check if 2 points can see each other
    pub fn check_visible_point(&self, origin: Vec3, target: Vec3) -> bool {
        let direction = (target - origin).normalized();
        let mut position = origin;
        for _ in 0..MAX_ITERATIONS {
            let dist_to_geometry = self.signed_distance(position);
            if dist_to_geometry < EPSILON {
                return false;
            }
            let dist_to_target = (target - origin).mag();
            if dist_to_target <= dist_to_geometry {
                return true;
            }

            position += dist_to_geometry * direction;
        }
        return false;
    }

    /// Check if a point can see to infinity in a certain direction
    pub fn check_visible_direction(&self, origin: Vec3, direction: Vec3) -> bool {
        let mut position = origin;
        for _ in 0..MAX_ITERATIONS {
            let dist_to_geometry = self.signed_distance(position);
            if dist_to_geometry < EPSILON {
                return false;
            }
            if dist_to_geometry > MAX_DISTANCE {
                return true;
            }

            position += dist_to_geometry * direction;
        }
        return true;
    }
}
