use std::error::Error;
use crossterm::QueueableCommand;

mod utils;
mod engine;
mod scene;
mod display;

use crate::engine::*;
use crate::scene::*;
use crate::display::*;

fn main() -> Result<(), Box<dyn Error>> {
    // Set up terminal
    let mut stdout = std::io::stdout();
    stdout
        .queue(crossterm::cursor::Hide)?
        .queue(crossterm::terminal::Clear(crossterm::terminal::ClearType::All))?;

    let camera = Camera {
        fov: 60.0 * 3.14159 / 180.0,
        screen_width : 96,
        screen_height: 32,
        pixel_aspect : 2.0284,
        transform    : Isometry3 {
            translation : Vec3::new(0.0, 0.0, -30.0),
            rotation    : Rotor3::identity(),
        }
    };

    let mut time: f32 = 0.0;
    loop {
        time += 0.1;

        // Recreate the scene every step cuz I'm lazy
        let scene = Box::new(MultiUnion {
            objects: vec![
                Box::new(Transform {
                    transform: Similarity3::new(
                        Vec3::new(-6.0, 0.0, 8.0), 
                        Rotor3::from_euler_angles(0.0, time, time*0.5), 
                        1.0),
                    object: Box::new(Torus {
                        radius_1: 5.0,
                        radius_2: 1.0,
                        material: Material {
                            color: Vec3::new(1.0, 0.2, 0.2),
                            reflective: 0.0,
                        },
                    })
                }),
                Box::new(Transform {
                    transform: Similarity3::new(
                        Vec3::new(5.0, 0.0, 10.0), 
                        Rotor3::from_euler_angles(time*0.4, time*0.25, time*0.6), 
                        1.0),
                    object: Box::new(Cuboid {
                        size: Vec3::new(6.0, 3.0, 3.0),
                        material: Material {
                            color: Vec3::new(0.2, 0.2, 1.0),
                            reflective: 0.5,
                        },
                    })
                }),
                Box::new(Transform {
                    transform: Similarity3::new(
                        Vec3::new(time.sin() * 12.0 + 5.0, time.cos() * 12.0, 20.0), 
                        Rotor3::identity(), 
                        1.0),
                    object: Box::new(Sphere {
                        radius: 8.0,
                        material: Material {
                            color: Vec3::broadcast(1.0),
                            reflective: 0.7,
                        },
                    })
                }),
                Box::new(Plane {
                    material: Material {
                        color: Vec3::new(0.2, 1.0, 0.2),
                        reflective: 0.5,
                    },
                    normal: Vec3::unit_y(),
                    height: 10.0,
                }),
            ],
        });

        // We need lights as well...
        let lights: Vec<Box<dyn Light>> = vec![
            // Global light makes it more visible, but also more ugly...
            //Box::new(GlobalLight {
            //    color: Vec3::one() * 0.0,
            //}),
            Box::new(PointLight {
                color   : Vec3::one(),
                position: Vec3::new(time.sin() * 5.0, 5.0, time.cos() * 5.0)
            }),
            Box::new(PointLight {
                color   : Vec3::new(1.0, 1.0, 1.0),
                position: Vec3::new(time.sin() * 3.0 - 15.0, 0.0, 8.0)
            }),
            Box::new(DirectionalLight {
                color    : Vec3::new(0.5, 0.5, 0.5),
                direction: Vec3::new(0.0, 0.0, -1.0),
            })
        ];

        let pixels = camera.render(scene, lights);
        draw_image(pixels, camera.screen_width, &mut stdout)?;
        
        // Do not sleep thread since we're performance bounded on console IO anyway...
    }
}
