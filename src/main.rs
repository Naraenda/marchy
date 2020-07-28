use ultraviolet :: {
    Vec3, Rotor3, Similarity3
};

const EPSILON        : f32   = 0.001;
const EPSILON_INVERSE: f32   = 1.0 / EPSILON;
const MAX_ITERATIONS : usize = 64;

fn main() {
    let width : i32 = 64;
    let height: i32 = 32;
    let fov : f32 = 30.0;
    let near : f32 = 1.5;
    let pixel_delta = fov.tan() * near as f32 * 0.01;
    let char_width_modifier = 0.75;

    let mut time: f32 = 0.0;
    loop {
        time += 0.1;
        let scene = SmoothUnion {
            smoothness: 1.2,
            object_a: Box::new(Transform {
                transform: Similarity3::new(
                    Vec3::new(-2.0, 0.0, 3.0), 
                    Rotor3::from_euler_angles(0.0, time, time*0.5), 
                    1.0),
                object: Box::new(Torus {
                    radius_1: 3.0,
                    radius_2: 0.9,
                })
            }),
            object_b: Box::new(Transform {
                transform: Similarity3::new(
                    Vec3::new(1.5, 1.0, 4.5), 
                    Rotor3::from_euler_angles(time*0.2, time*0.25, time*0.6), 
                    1.0),
                object: Box::new(Cuboid {
                    size: Vec3::new(5.0, 2.0, 2.0),
                })
            })
        };

        // Create empty canvas
        let mut pixels: Vec<f32> = vec![0.0; (width * height) as usize];

        let origin = Vec3::new(0.0, 0.0, -3.0);
        for pixel_y in 0..height {
            let y = (pixel_y - height / 2) as f32 * pixel_delta;
            for pixel_x in 0..width {
                let x = (pixel_x - width / 2) as f32 * pixel_delta * char_width_modifier;
                let direction = Vec3::new(x, y, near).normalized();

                // Ray march and draw to canvas
                let result = (&scene as &dyn SignedDistance).ray_march(origin, direction);
                pixels[(pixel_y * width + pixel_x) as usize] = result;
            }
        }
        
        // Consume pixels, produce ascii art
        let image = pixels.into_iter().map(|p| quantize_to_char(p)).collect::<Vec<char>>();
        for row in image.chunks(width as usize) {
            let line: String = row.into_iter().collect();
            println!("{}", line)
        }
    }
}

/// Converts boring floats into cool ascii.
fn quantize_to_char(value: f32) -> char {
    if value > 0.9 { 
        '#'
    } else if value > 0.75 {
        '+'
    } else if value > 0.5 {
        '/'
    } else if value > 0.10 {
        ':'
    } else if value > 0.0 {
        '.'
    } else { 
        ' '
    }
}

trait SignedDistance {
    /// Returns the signed distances from a point. Negative means we are inside something.
    fn signed_distance(&self, point: Vec3) -> f32;
}

impl dyn SignedDistance {
    /// Normal vector from a point near the surface described by the signed distance function.
    fn normal(&self, point: Vec3) -> Vec3 {
        let distance_x: f32 = self.signed_distance(point + Vec3::new(EPSILON, 0.0, 0.0));
        let distance_y: f32 = self.signed_distance(point + Vec3::new(0.0, EPSILON, 0.0));
        let distance_z: f32 = self.signed_distance(point + Vec3::new(0.0, 0.0, EPSILON));
        ((Vec3::new(distance_x, distance_y, distance_z) - point) * EPSILON_INVERSE).normalized()
    }

    /// Ray marches from `origin` towards `direction` using a maximum of `MAX_ITERATIONS` steps.
    fn ray_march(&self, origin: Vec3, direction: Vec3) -> f32 {
        let mut position = origin;

        // Hard coded lights, should probably remove...
        let dir_light_1: Vec3 = Vec3::new(2.0, -2.0, -1.0).normalized();
        let dir_light_2: Vec3 = Vec3::new(0.0, 1.0, 1.0).normalized();

        for _ in 0..MAX_ITERATIONS {
            let distance = self.signed_distance(position);
            if distance < EPSILON {
                // Use dot product for simple diffuse shading
                let mut intensity = 
                    Vec3::dot(&self.normal(position), dir_light_1) * 1.3 + 
                    Vec3::dot(&self.normal(position), dir_light_2);
                intensity = intensity.max(0.1);
                return intensity;
            }
            position = position + direction * distance;
        }

        // Could not reach anything :(
        return 0.0;
    }
}

/// Combines two SDFs by taking the minimum distance.
struct Union { 
    object_a: Box<dyn SignedDistance>,
    object_b: Box<dyn SignedDistance>,
}

impl SignedDistance for Union {
    fn signed_distance(&self, point: Vec3) -> f32 {
        f32::min(self.object_a.signed_distance(point), self.object_b.signed_distance(point))
    }
}

/// Like `Union` but more epic
struct SmoothUnion {
    object_a: Box<dyn SignedDistance>,
    object_b: Box<dyn SignedDistance>,
    smoothness: f32,
}

impl SignedDistance for SmoothUnion {
    fn signed_distance(&self, point: Vec3) -> f32 {
        let k  = self.smoothness;
        let d1 = self.object_a.signed_distance(point);
        let d2 = self.object_b.signed_distance(point);
        let h  = (0.5 + 0.5 * (d2 - d1) / k).max(0.0).min(1.0);
        d1 * h + d2 * (1.0 - h) - k * h * (1.0 - h)
    }
}

/// Allows the scaling, rotation and translation of objects.
struct Transform {
    object: Box<dyn SignedDistance>,
    transform: Similarity3
}

impl SignedDistance for Transform {
    fn signed_distance(&self, point: Vec3) -> f32 {
         self.object.signed_distance(self.transform.inversed() * point)
    }
}

struct Cuboid {
    size: Vec3,
}

impl SignedDistance for Cuboid {
    fn signed_distance(&self, point: Vec3) -> f32 {
        let q = point.abs() - self.size;
        q.max_by_component(Vec3::zero()).mag() + f32::min(q.component_max(), 0.0)
    }
}

struct Sphere {
    radius: f32,
}

impl SignedDistance for Sphere {
    fn signed_distance(&self, point: Vec3) -> f32 {
        point.mag() - self.radius
    }
}

/// Donut? yes.
struct Torus {
    /// Outer radius
    radius_1: f32,

    /// Inner radius
    radius_2: f32,
}

impl SignedDistance for Torus {
    fn signed_distance(&self, point: Vec3) -> f32 {
        let a = (point.x * point.x + point.z * point.z).sqrt() - self.radius_1;
        let b = (a * a + point.y * point.y).sqrt();
        return b - self.radius_2;
    }
}
