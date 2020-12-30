mod sphere;
pub use sphere::Sphere;

mod ray;
pub use ray::Ray;

pub trait Scene {
    /// Return the normal to the first intersection, if it exists.
    fn intersect(&self, ray: &Ray) -> Option<Ray>;
}
