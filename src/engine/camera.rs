use crate::engine::*;

pub struct Camera {
    pub fov: f32,
    pub screen_width : i32,
    pub screen_height: i32,
    pub pixel_aspect : f32,
    pub transform    : Isometry3,
}

impl Camera {
    pub fn render(&self, scene: Box<dyn Marchable>, lights: Vec<Box<dyn Light>>) -> Vec<Vec3> {
        let origin = self.transform.translation;

        let mut pixels: Vec<Vec3> = vec![Vec3 { x: 0.0, y: 0.0, z: 0.0 }; (self.screen_width * self.screen_height) as usize];

        let width = 2.0 * (self.fov * 0.5).tan();
        let dist_per_pixel = width / (self.screen_width as f32);

        for pixel_y in 0..self.screen_height {
            // We start at the top so we need to count down
            let y = -(pixel_y - self.screen_height / 2) as f32 * dist_per_pixel * self.pixel_aspect;
            for pixel_x in 0..self.screen_width {
                let x = (pixel_x - self.screen_width / 2) as f32 * dist_per_pixel;

                let direction = Vec3::new(x, y, 1.0).normalized();
                let result = scene.ray_march(origin, direction, &lights, 0);

                pixels[(pixel_y * self.screen_width + pixel_x) as usize] = result;
            }
        }

        return pixels;
    }
}
