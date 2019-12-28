use super::Ray;
use cgmath::Vector3;

pub trait Scene {
    /// Return the distance to the first intersection, if it exists.
    fn intersect(&self, ray: &Ray) -> Option<f64>;
    fn normal(&self, point: Vector3<f64>) -> Vector3<f64>;
}

mod sphere;
pub use sphere::Sphere;