mod sphere;
pub use sphere::Sphere;

mod triangle;
pub use triangle::Triangle;

use super::math;
use math::Ray;

pub trait Scene {
    /// Return the normal to the first intersection, if it exists.
    fn intersect(&self, ray: &Ray) -> Option<Ray>;
}
