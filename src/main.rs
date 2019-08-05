extern crate cgmath;
extern crate image;

use cgmath::{Deg, InnerSpace, Vector3, vec3};
use image::{ImageBuffer, Rgb, RgbImage};
use std::f64::consts::PI;

mod camera;
use camera::CameraMatrix;

struct Ray {
    origin: Vector3<f64>,
    direction: Vector3<f64>,
}

impl Ray {
    pub fn new(origin: Vector3<f64>, direction: Vector3<f64>) -> Ray {
        // Avoid weirdness by always normalizing ray directions
        Ray {
            origin,
            direction: direction.normalize(),
        }
    }
}

struct Sphere {
    center: Vector3<f64>,
    radius: f64,
}

impl Sphere {
    // Returns the distance to the first intersection with a sphere, if it
    // exists.
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let tmp = ray.origin - self.center;
        let b = 2.0 * tmp.dot(ray.direction);
        let c = tmp.dot(tmp) - self.radius.powi(2);

        let discriminant = b.powi(2) - 4.0 * c;

        if discriminant >= 0.0 {
            let discriminant = discriminant.sqrt();
            let solution_0 = -b - discriminant;
            let solution_1 = -b + discriminant;
            if solution_0 >= 0.0 {
                return Some(solution_0 / 2.0);
            } else if solution_1 >= 0.0 {
                return Some(solution_1 / 2.0);
            }
        }

        None
    }

    fn normal(&self, point: Vector3<f64>) -> Vector3<f64> {
        let x = (point.x - self.center.x) / self.radius;
        let y = (point.y - self.center.y) / self.radius;
        let z = (point.z - self.center.z) / self.radius;

        vec3(x, y, z)
    }
}

fn main() {
    const IMAGE_WIDTH: u32 = 800;
    const IMAGE_HEIGHT: u32 = 600;

    let mut image: RgbImage = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    let fov = Deg(100.0);
    //let cam = CameraMatrix::new(fov, IMAGE_WIDTH, IMAGE_HEIGHT);
    let origin = vec3(0.0, 0.0, 0.0);

    for j in 0..IMAGE_HEIGHT {
        for i in 0..IMAGE_WIDTH {
            //let dir = cam * vec2(i as f64, j as f64);
            let dir =
                CameraMatrix::camera(fov, IMAGE_WIDTH, IMAGE_HEIGHT, i, j);

            let ray = Ray::new(origin, dir);
            let light_intensity = trace(ray);
            let I = f64::floor(light_intensity * 255.0) as u8;
            image[(i,j)] = Rgb([I, I, I]);
        }
    }

    image.save("render.png").expect("Failed to write image");
}

const LIGHT: Vector3<f64> = vec3(5.0, 5.0, 0.0);
const LIGHT_ENERGY: f64 = 500.0;
const AMBIENT_LIGHT: f64 = 0.01;
const SPHERE: Sphere = Sphere {
    center: vec3(0.0, 0.0, -10.0),
    radius: 5.0,
};


// TODO: color encoding problem; light intensity is not bound to the range
// [0.0, 1.0]. The image generation code assumes this in order to convert to the
// range [0, 256) for image encoding, causing overflow errors in the final image
// when the value goes above 255.
fn trace(ray: Ray) -> f64 {
    if let Some(t) = SPHERE.intersect(&ray) {
        let hit = ray.origin + ray.direction * t;
        let normal = SPHERE.normal(hit);

        let light_vec = LIGHT - hit;
        let magnitude2 = light_vec.magnitude2();

        let lambert = Vector3::dot(normal, light_vec) / f64::sqrt(magnitude2);
        let intensity = lambert * LIGHT_ENERGY / (4.0 * PI * magnitude2);
        let intensity = f64::max(intensity, AMBIENT_LIGHT);
        return intensity;
    }

    0.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ray_sphere_intersect() {
        let sphere = Sphere {
            center: vec3(0.0, 0.0, 0.0),
            radius: 5.0,
        };

        let point = vec3(0.0, 0.0, 10.0);
        let direction = vec3(0.0, 0.0, -1.0);
        let ray = Ray::new(point, direction);

        let distance = sphere.intersect(&ray);
        assert_eq!(distance.unwrap(), 5.0);
    }
}
