use ultraviolet::vec::DVec4;
use ultraviolet::mat::DMat4;
use std::ops::{Add, Mul};

pub const fn vector(x: f64, y: f64, z: f64) -> DVec4 {
    DVec4::new(x, y, z, 0.0)
}

pub const fn point(x: f64, y: f64, z: f64) -> DVec4 {
    DVec4::new(x, y, z, 1.0)
}

pub const fn translation(x: f64, y: f64, z: f64) -> DMat4 {
    let c0 = DVec4::new(1.0, 0.0, 0.0, 0.0);
    let c1 = DVec4::new(0.0, 1.0, 0.0, 0.0);
    let c2 = DVec4::new(0.0, 0.0, 1.0, 0.0);
    let c3 = DVec4::new(x, y, z, 1.0);
    DMat4::new(c0, c1, c2, c3)
}

#[derive(Debug, PartialEq)]
pub struct Ray {
    pub origin: DVec4,
    pub direction: DVec4,
}

impl Ray {
    pub fn new(origin: DVec4, direction: DVec4) -> Ray {
        Ray { origin, direction }
    }

    pub fn position(&self, t: f64) -> DVec4 {
        self.origin + self.direction * t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_ray() {
        let origin = point(1.0, 2.0, 3.0);
        let direction = vector(4.0, 5.0, 6.0);
        let ray = Ray::new(origin.clone(), direction.clone());
        assert_eq!(ray.origin, origin);
        assert_eq!(ray.direction, direction);
    }

    #[test]
    fn test_compute_position_from_ray() {
        let origin = point(2.0, 3.0, 4.0);
        let direction = vector(1.0, 0.0, 0.0);
        let ray = Ray::new(origin, direction);
        assert_eq!(ray.position(0.0), point(2.0, 3.0, 4.0));
        assert_eq!(ray.position(1.0), point(3.0, 3.0, 4.0));
        assert_eq!(ray.position(-1.0), point(1.0, 3.0, 4.0));
        assert_eq!(ray.position(2.5), point(4.5, 3.0, 4.0));
    }

    #[test]
    fn test_translation_mat() {
        let transform = translation(5.0, -3.0, 2.0);
        let p = point(-3.0, 4.0, 5.0);
        assert_eq!(transform * p, point(2.0, 1.0, 7.0));
    }

    #[test]
    fn test_translation_mat_inverse() {
        let mut transform = translation(5.0, -3.0, 2.0);
        transform.inverse();
        let p = point(-3.0, 4.0, 5.0);
        assert_eq!(transform * p, point(-8.0, 7.0, 3.0))
    }

    #[test]
    fn test_translation_doesnt_work_on_vectors() {
        let mut transform = translation(5.0, -3.0, 2.0);
        let v = vector(-3.0, 4.0, 5.0);
        assert_eq!(transform * v, v);
    }
}

#[cfg(test)]
pub mod test_util {
    // floating point comparison utilities
    use ultraviolet::{DVec3, DVec4};
    use std::fmt::Debug;

    pub trait EpsEq<Rhs = Self> {
        type Rhs;
        fn eps_eq(&self, rhs: &Self::Rhs, eps: f64) -> bool;
    }

    impl EpsEq for DVec4 {
        type Rhs = Self;
        fn eps_eq(&self, rhs: &Self::Rhs, eps: f64) -> bool {
            (*self - *rhs).abs().component_max() < eps
        }
    }

    impl EpsEq for DVec3 {
        type Rhs = Self;
        fn eps_eq(&self, rhs: &Self::Rhs, eps: f64) -> bool {
            (*self - *rhs).abs().component_max() < eps
        }
    }

    impl EpsEq for f64 {
        type Rhs = Self;
        fn eps_eq(&self, rhs: &Self::Rhs, eps: f64) -> bool {
            (self - rhs).abs() < eps
        }
    }

    impl<T> EpsEq for (T, T)
    where
        T: EpsEq<Rhs = T>,
    {
        type Rhs = Self;
        fn eps_eq(&self, rhs: &Self::Rhs, eps: f64) -> bool {
            self.0.eps_eq(&rhs.0, eps) && self.1.eps_eq(&rhs.1, eps)
        }
    }

    impl<T> EpsEq for Option<T>
    where
        T: EpsEq<Rhs = T>,
    {
        type Rhs = Self;
        fn eps_eq(&self, rhs: &Self::Rhs, eps: f64) -> bool {
            match (self, rhs) {
                (Some(a), Some(b)) => a.eps_eq(&b, eps),
                (None, None) => true,
                _ => false,
            }
        }
    }

    pub fn assert_eps_eq<T>(a: &T, b: &T, eps: f64)
    where
        T: EpsEq<Rhs = T> + Debug,
    {
        assert!(a.eps_eq(b, eps), "{:?} != {:?}, eps = {}", a, b, eps);
    }
}
