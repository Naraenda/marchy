use ultraviolet::Similarity3;
use ultraviolet::Vec3;
use crate::utils::Mixable;
use crate::engine::marcher::*;
use crate::engine::material::*;

/// Combines two SDFs by taking the minimum distance.
pub struct Union { 
    pub object_a: Box<dyn Marchable>,
    pub object_b: Box<dyn Marchable>,
}

impl Marchable for Union {
    fn signed_distance(&self, point: Vec3) -> f32 {
        f32::min(self.object_a.signed_distance(point), self.object_b.signed_distance(point))
    }

    fn dist_material(&self, point: Vec3) -> (f32, Material) {
        let (dist_a, mat_a) = self.object_a.dist_material(point);
        let (dist_b, mat_b) = self.object_b.dist_material(point);
        if dist_a < dist_b { 
            (dist_a, mat_a) 
        } else { 
            (dist_b, mat_b) 
        }
    }
}

pub struct MultiUnion {
    pub objects: Vec<Box<dyn Marchable>>
}

impl Marchable for MultiUnion {
    fn signed_distance(&self, point: Vec3) -> f32 {
        self.objects
            .iter()
            .map(|m| m.signed_distance(point))
            .min_by(|a, b| a.partial_cmp(b).expect("Tried to compare a NaN"))
            .unwrap()
    }

    fn dist_material(&self, point: Vec3) -> (f32, Material) {
        self.objects
            .iter()
            .map(|m| m.dist_material(point))
            .min_by(|(a, _), (b, _)| a.partial_cmp(b).expect("Tried to compare a NaN"))
            .unwrap()
    }
}

/// Like `Union` but more epic
pub struct SmoothUnion {
    pub object_a: Box<dyn Marchable>,
    pub object_b: Box<dyn Marchable>,
    pub smoothness: f32,
}

impl Marchable for SmoothUnion {
    fn signed_distance(&self, point: Vec3) -> f32 {
        let k  = self.smoothness;
        let d1 = self.object_a.signed_distance(point);
        let d2 = self.object_b.signed_distance(point);
        let h  = (0.5 + 0.5 * (d2 - d1) / k).max(0.0).min(1.0);
        d1 * h + d2 * (1.0 - h) - k * h * (1.0 - h)
    }

    fn dist_material(&self, point: Vec3) -> (f32, Material) {
        let k  = self.smoothness;
        let (dist_a, mat_a) = self.object_a.dist_material(point);
        let (dist_b, mat_b) = self.object_b.dist_material(point);
        let h  = (0.5 + 0.5 * (dist_b - dist_a) / k).max(0.0).min(1.0);
        let dist = Mixable::mix(dist_b, dist_a, h) * (1.0 - h) - k * h * (1.0 - h);
        let mat  = Mixable::mix(mat_b , mat_a , h);
        (dist, mat)
    }
}

/// Allows the scaling, rotation and translation of objects.
pub struct Transform {
    pub object: Box<dyn Marchable>,
    pub transform: Similarity3
}

impl Marchable for Transform {
    fn signed_distance(&self, point: Vec3) -> f32 {
         self.object.signed_distance(self.transform.inversed() * point)
    }

    fn dist_material(&self, point: Vec3) -> (f32, Material) {
        self.object.dist_material(self.transform.inversed() * point)
    }
}
