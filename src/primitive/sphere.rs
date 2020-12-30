use super::Scene;
use super::math::{Ray, vector};
use ultraviolet::DVec4;

pub struct Sphere {
    center: DVec4,
    radius: f64,
}

impl Sphere {
    pub fn new(center: DVec4, radius: f64) -> Sphere {
        Sphere { center, radius }
    }

    fn normal(&self, point: DVec4) -> DVec4 {
        let x = (point.x - self.center.x) / self.radius;
        let y = (point.y - self.center.y) / self.radius;
        let z = (point.z - self.center.z) / self.radius;

        vector(x, y, z)
    }

    fn solve_intersect(&self, ray: &Ray) -> Option<(f64, f64)> {
        let sphere_to_ray = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - self.radius * self.radius;
        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant >= 0.0 {
            let discriminant = discriminant.sqrt();
            let denominator = 2.0 * a;
            let solution_a = (-b - discriminant) / denominator;
            let solution_b = (-b + discriminant) / denominator;
            Some((solution_a, solution_b))
        } else {
            None
        }
    }
}

impl Scene for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Ray> {
        if let Some((solution_a, solution_b)) = self.solve_intersect(ray) {
            let mut dist = None;
            if solution_a >= 0.0 {
                dist = Some(solution_a / 2.0);
            } else if solution_b >= 0.0 {
                dist = Some(solution_b / 2.0);
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
    use super::super::math;
    use math::point;
    use math::test_util::assert_eps_eq;

    #[test]
    fn test_ray_sphere_intersect() {
        let sphere = Sphere::new(point(0.0, 0.0, 0.0), 1.0 /* radius */);
        let ray = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));

        let res = sphere.solve_intersect(&ray);
        assert_eps_eq(&res, &Some((4.0, 6.0)), 0.01);
    }
}
