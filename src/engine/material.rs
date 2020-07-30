use crate::engine::*;
use crate::utils::Mixable;

#[derive(Copy, Clone)]
pub struct Material {
    pub color: Vec3,
    pub reflective: f32,
}

impl Mixable for Material {
    fn mix(self, other: Material, weight: f32) -> Self { 
        Material { 
            color: Mixable::mix(self.color, other.color, weight),
            reflective: Mixable::mix(self.reflective, other.reflective, weight),
        }
    }
}
