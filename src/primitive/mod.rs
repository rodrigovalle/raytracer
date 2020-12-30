mod sphere;
pub use sphere::Sphere;

use super::math;
use math::Ray;

pub trait Scene {
    /// Return the normal to the first intersection, if it exists.
    fn intersect(&self, ray: &Ray) -> Option<Ray>;
}
