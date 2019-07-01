extern crate cgmath;
extern crate image;

use cgmath::{Deg, InnerSpace, Vector3, vec3};
use image::{ImageBuffer, Rgb, RgbImage};

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
            image[(i, j)] = trace(ray);
        }
    }

    image.save("render.png").expect("Failed to write image");
}

const LIGHT: Vector3<f64> = vec3(5.0, 5.0, 0.0);
const SPHERE: Sphere = Sphere {
    center: vec3(0.0, 0.0, -10.0),
    radius: 5.0,
};

fn trace(ray: Ray) -> Rgb<u8> {
    if let Some(t) = SPHERE.intersect(&ray) {
        let hit = ray.origin + ray.direction * t;
        let normal = SPHERE.normal(hit);
        let light_dir = LIGHT - hit;
        let light_dir = light_dir.normalize();

        let brightness = normal.dot(light_dir);
        let brightness = brightness * 255.0;
        let brightness = brightness.max(0.0).floor() as u8;

        return Rgb([brightness, brightness, brightness]);
    }

    Rgb([0, 0 ,0])
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

        // direction must be normalized otherwise we get weird intersect results
        let point = vec3(0.0, 0.0, 10.0);
        let direction = vec3(0.0, 0.0, -1.0);
        let ray = Ray::new(point, direction);

        let distance = sphere.intersect(ray);
        assert_eq!(distance.unwrap(), 5.0);
    }
}
