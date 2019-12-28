use super::{Scene, Ray};
use cgmath::{InnerSpace, Vector3, vec3};

pub struct Sphere {
    center: Vector3<f64>,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vector3<f64>, radius: f64) -> Sphere {
        Sphere { center, radius }
    }
}

impl Scene for Sphere {
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
