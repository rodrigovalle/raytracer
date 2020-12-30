use super::{Scene, Ray};
use ultraviolet::DVec3;

pub struct Sphere {
    center: DVec3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: DVec3, radius: f64) -> Sphere {
        Sphere { center, radius }
    }

    fn normal(&self, point: DVec3) -> DVec3 {
        let x = (point.x - self.center.x) / self.radius;
        let y = (point.y - self.center.y) / self.radius;
        let z = (point.z - self.center.z) / self.radius;

        DVec3::new(x, y, z)
    }
}

impl Scene for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Ray> {
        let tmp = ray.origin - self.center;
        let b = 2.0 * tmp.dot(ray.direction);
        let c = tmp.dot(tmp) - self.radius.powi(2);

        let discriminant = b.powi(2) - 4.0 * c;

        if discriminant >= 0.0 {
            let discriminant = discriminant.sqrt();
            let solution_0 = -b - discriminant;
            let solution_1 = -b + discriminant;

            let mut dist = None;
            if solution_0 >= 0.0 {
                dist = Some(solution_0 / 2.0);
            } else if solution_1 >= 0.0 {
                dist = Some(solution_1 / 2.0);
            }

            if let Some(t) = dist {
                let intersect = ray.position(t);
                let normal = self.normal(intersect);
                return Some(Ray::new(intersect, normal));
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ray_sphere_intersect() {
        // sphere centered at (0, 0, 0) with a radius of 5
        let sphere = Sphere::new(DVec3::new(0.0, 0.0, 0.0), 5.0);

        let point = DVec3::new(0.0, 0.0, 10.0);
        let direction = DVec3::new(0.0, 0.0, -1.0);
        let ray = Ray::new(point, direction);

        let normal = sphere.intersect(&ray).unwrap();
        assert_eq!(normal.origin, DVec3::new(0.0, 0.0, 5.0));
        assert_eq!(normal.direction, DVec3::new(0.0, 0.0, 1.0));
    }
}
