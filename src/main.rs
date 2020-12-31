extern crate image;

use ultraviolet::vec::DVec4;
use image::{ImageBuffer, Rgb, RgbImage};
use std::f64::consts::PI;

mod camera;
mod color;
mod math;
mod primitive;
use math::{Ray, point, vector};
use primitive::{Scene, Sphere};

fn main() {
    const IMAGE_WIDTH: u32 = 800;
    const IMAGE_HEIGHT: u32 = 600;

    let mut image: RgbImage = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    // camera is facing in the -z direction
    let fov = f64::to_radians(100.0);
    let camera = camera::projection_matrix(fov, IMAGE_WIDTH, IMAGE_HEIGHT);
    let origin = point(0.0, 0.0, 0.0);
    let scene = Sphere::new(point(0.0, 0.0, -3.0), 1.0);

    for j in 0..IMAGE_HEIGHT {
        for i in 0..IMAGE_WIDTH {
            let dir = camera * vector(i as f64, j as f64, 1.0);
            let ray = Ray::new(origin, dir);
            let light_intensity = trace(ray, &scene);
            let I = f64::floor(light_intensity * 255.0) as u8;
            image[(i, j)] = Rgb([I, I, I]);
        }
    }

    image.save("render.png").expect("Failed to write image");
}

const LIGHT: DVec4 = point(-3.0, -3.0, 0.0);
const LIGHT_POWER: f64 = 300.0;
const AMBIENT_LIGHT: f64 = 0.03;

// TODO: color encoding problem; light intensity is not bound to the range
// [0.0, 1.0]. The image generation code assumes this in order to convert to the
// range [0, 256) for image encoding, causing overflow errors in the final image
// when the value goes above 255.
fn trace(ray: Ray, scene: &impl Scene) -> f64 {
    if let Some(normal) = scene.intersect(&ray) {
        let light_vec = normal.origin - LIGHT;  // point - point is a vector
        let light_mag_sq = light_vec.mag_sq();

        let cos = normal.direction.dot(light_vec) / (normal.direction.mag() * f64::sqrt(light_mag_sq));
        let intensity =  LIGHT_POWER * cos / (4.0 * PI * light_mag_sq);
        return f64::max(intensity, AMBIENT_LIGHT);
    }

    0.0
}
