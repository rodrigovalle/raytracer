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
        point - self.center / self.radius
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
            // TODO: we can avoid a division if we know the answer is going to be negative and
            // we're going to discard it anyways because the intersection is behind the ray. The
            // denominator's sign is always positive so it won't change the sign of the result.
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
            let dist = solution_a.max(solution_b);
            if dist >= 0.0 {
                let intersect = ray.position(dist);
                let normal = self.normal(intersect);
                Some(Ray::new(intersect, normal))
            } else {
                None
            }
        } else {
            None
        }
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

    #[test]
    fn test_ray_sphere_normal() {
        let sphere = Sphere::new(point(0.0, 0.0, 0.0), 1.0 /* radius */);
        let point_on_sphere = point(0.0, 0.0, 1.0);
        assert_eps_eq(&sphere.normal(point_on_sphere), &vector(0.0, 0.0, 1.0), 0.01);
    }
}
