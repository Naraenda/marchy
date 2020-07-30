use crate::engine::*;

#[inline]
fn shade(fragment: &Fragment, light_direction: Vec3, light_color: Vec3) -> Vec3 {
    let luminance = light_direction.dot(fragment.normal).max(0.0);
    fragment.material.color * light_color * luminance
}

pub trait Light {
    fn luminate(&self, fragment: &Fragment, world: &dyn Marchable) -> Vec3;
}

pub struct PointLight {
    pub color   : Vec3,
    pub position: Vec3,
}

impl Light for PointLight {
    fn luminate(&self, fragment: &Fragment, world: &dyn Marchable) -> Vec3 {
        let light_direction = (self.position - fragment.position).normalized();

        // If no light transfer possible, return zero light.
        if !world.check_visible_point(fragment.position + light_direction * EPSILON * 2.0, self.position) {
            return Vec3::zero();
        }

        // Use squared distance for light falloff, adjust with magic number to make the scene not too dark
        64.0 * shade(fragment, light_direction, self.color) / (self.position - fragment.position).mag_sq()
    }
}

pub struct DirectionalLight {
    pub color    : Vec3,
    pub direction: Vec3,
}

impl Light for DirectionalLight {
    fn luminate(&self, collision: &Fragment, world: &dyn Marchable) -> Vec3 {
        let light_direction = (-self.direction).normalized();

        // If no light transfer possible, return zero light.
        if !world.check_visible_direction(collision.position + light_direction * EPSILON * 2.0, light_direction) {
            return Vec3::zero();
        }

        shade(collision, light_direction, self.color)
    }
}

pub struct GlobalLight {
    pub color: Vec3
}

impl Light for GlobalLight {
    fn luminate(&self, _: &Fragment, _: &dyn Marchable) -> Vec3 {
        self.color
    }
}
