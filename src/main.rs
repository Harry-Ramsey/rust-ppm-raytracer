pub mod vec3;
pub mod colour;
pub mod ray;
pub mod camera;
pub mod sphere;
pub mod hittable;

use crate::hittable::Hittable;
use crate::vec3::Vec3;
use crate::colour::Colour;
use crate::camera::Camera;
use crate::sphere::Sphere;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as u32;

    let camera = Camera::new(
        Vec3 { x: 0.0, y: 0.0, z: 0.0 },
        Vec3 { x: 0.0, y: 0.0, z: -1.0 },
        Vec3 { x: 0.0, y: 1.0, z: 0.0 },
        90.0,
        aspect_ratio,
    );

    let sphere = Sphere { centre: Vec3 {x: 0.0, y: 0.0, z: -1.0}, radius: 0.5, colour: Colour::new(0.0, 0.0, 1.0) };

    println!("P3\n{} {}\n255", image_width, image_height);

    for x in (0..image_height).rev() {
        for y in 0..image_width {
            // Transformed x and y into viewport vectors
            let u = y as f64 / (image_width - 1) as f64;
            let v = x as f64 / (image_height - 1) as f64;

            let ray = camera.get_ray(u, v);
            let unit_direction = ray.direction.unit_vector();

            let pixel_colour;
            match sphere.hit(ray) {
                Some(colour) => {
                    pixel_colour = colour;
                },
                None => {
                    let t = 0.5 * (unit_direction.y + 1.0);
                    // A simple gradient from white to blue
                    let c_vec = Vec3 { x: 1.0, y: 1.0, z: 1.0 } * (1.0 - t) + Vec3 { x: 0.5, y: 0.7, z: 1.0 } * t;
                    pixel_colour = Colour(c_vec);
                }
            }
            println!("{}", pixel_colour);
        }
    }
}
