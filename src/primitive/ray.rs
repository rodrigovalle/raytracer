use ultraviolet::vec::DVec3;

#[derive(Debug, PartialEq)]
pub struct Ray {
    pub origin: DVec3,
    pub direction: DVec3
}

impl Ray {
    pub fn new(origin: DVec3, mut direction: DVec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn position(&self, t: f64) -> DVec3 {
        self.origin + self.direction * t
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_ray() {
        let origin = DVec3::new(1.0, 2.0, 3.0);
        let direction = DVec3::new(4.0, 5.0, 6.0);
        let ray = Ray::new(origin.clone(), direction.clone());
        assert_eq!(ray.origin, origin);
        assert_eq!(ray.direction, direction);
    }

    #[test]
    fn compute_position() {
        let origin = DVec3::new(2.0, 3.0, 4.0);
        let direction = DVec3::new(1.0, 0.0, 0.0);
        let ray = Ray::new(origin, direction);
        assert_eq!(ray.position(0.0), DVec3::new(2.0, 3.0, 4.0));
        assert_eq!(ray.position(1.0), DVec3::new(3.0, 3.0, 4.0));
        assert_eq!(ray.position(-1.0), DVec3::new(1.0, 3.0, 4.0));
        assert_eq!(ray.position(2.5), DVec3::new(4.5, 3.0, 4.0));
    }
}
