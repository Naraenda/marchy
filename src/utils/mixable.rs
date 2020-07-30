use ultraviolet::Vec3;

pub trait Mixable {
    fn mix(self, other: Self, weight: f32) -> Self;
}

impl Mixable for f32 {
    fn mix(self, other: f32, weight: f32) -> f32 {
        other * weight + self * (1.0 - weight)
    }
}

impl Mixable for Vec3 {
    fn mix(self, other: Vec3, weight: f32) -> Vec3 {
        other * weight + self * (1.0 - weight)
    }
}