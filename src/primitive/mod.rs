use super::Ray;

pub trait Scene {
    /// Return the normal to the first intersection, if it exists.
    fn intersect(&self, ray: &Ray) -> Option<Ray>;
}

mod sphere;
pub use sphere::Sphere;
