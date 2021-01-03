use super::Scene;
use super::math::{Ray, vector};

use ultraviolet::vec::{DVec3, DVec4};

pub struct Triangle {
    p0: DVec4,
    p1: DVec4,
    p2: DVec4,
    e0: DVec4,
    e1: DVec4,
    normal: DVec4,
}

impl Triangle {
    pub fn new(p0: DVec4, p1: DVec4, p2: DVec4) -> Triangle {
        // compute edge vectors
        let e0 = p1 - p0;
        let e1 = p2 - p0;

        // compute normal vector
        let mut normal = e1.xyz().cross(e0.xyz());
        normal.normalize();

        Triangle {
            p0,
            p1,
            p2,
            e0,
            e1,
            normal: normal.xyzw(),
        }
    }

    // TODO: stick this in an interface for primitive objects instead
    pub fn normal(&self, point: DVec4) -> DVec4 {
        self.normal
    }

    /// Returns the intersection point on the surface of the triangle if `ray` intersects.
    /// Uses the a geometric solution.
    // TODO: this is broken
    fn geometric_intersect(&self, ray: &Ray) -> Option<DVec4> {
        let denominator = self.normal.dot(ray.direction);
        // check that ray is not parallel to the plane
        if denominator == 0.0 {
            return None;
        }

        // solve for t in the ray equation
        let t = self.normal.dot(self.p0 - ray.origin) / denominator;
        // find the point on the surface of the triangle's plane
        let p = ray.position(t);
        if t < 0.0 {
            // intersect was behind the ray
            return None;
        }

        // inside outside tests to check if the intersection point is inside the triangle
        // calculate vectors relative to the p0 vertex of the triangle
        let c0 = p.xyz() - self.p0.xyz();
        let c1 = p.xyz() - self.p1.xyz();
        let c2 = p.xyz() - self.p2.xyz();

        // cross e0 and c0 and the resulting vector points in the same direction as the normal if
        // c0 is "inside" (to the left of) the v_n -> v_{n+1} edge
        if self.e0.xyz().cross(c0).dot(self.normal.xyz()) >= 0.0
            && self.e0.xyz().cross(c1).dot(self.normal.xyz()) >= 0.0
            && self.e1.xyz().cross(c2).dot(self.normal.xyz()) >= 0.0
        {
            Some(p)
        } else {
            None
        }
    }

    /// There are even faster algorithms for ray/triangle intersection but moller-trumbore is
    /// already faster than calculating the geometric solution and is a popular choice.
    /// see this for faster algorithms: https://stackoverflow.com/questions/44275153
    /// see this for an explanation of moller-trumbore: https://www.scratchapixel.com/lessons/3d-basic-rendering/ray-tracing-rendering-a-triangle/moller-trumbore-ray-triangle-intersection
    ///
    /// Returns the intersection point on the surface of the triangle if `ray` intersects.
    fn moller_trumbore_intersect(&self, ray: &Ray) -> Option<DVec4> {
        let E0 = self.e0.xyz();
        let E1 = self.e1.xyz();
        let P = ray.direction.xyz().cross(E1);
        let denominator = P.dot(E0);
        if denominator == 0.0 {  // if the denominator is < 0 then we hit the back of the triangle
            return None
        }

        let T = ray.origin.xyz() - self.p0.xyz();
        let coefficient = 1.0 / denominator;
        let u = coefficient * P.dot(T);
        if u < 0.0 || u > 1.0 {
            return None
        }

        let Q = T.cross(E0);
        let v = coefficient * Q.dot(ray.direction.xyz());
        if v < 0.0 || u + v > 1.0 {
            return None
        }

        let t = coefficient * Q.dot(E1);
        Some(ray.position(t))
    }
}

impl Scene for Triangle {
    fn intersect(&self, ray: &Ray) -> Option<Ray> {
        if let Some(intersect) = self.moller_trumbore_intersect(ray) {
            Some(Ray::new(intersect, self.normal(intersect)))
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

    const EPS: f64 = 0.01;

    #[test]
    fn test_triangle_construction() {
        let p0 = point(0.0, 1.0, 0.0);
        let p1 = point(-1.0, 0.0, 0.0);
        let p2 = point(1.0, 0.0, 0.0);
        let triangle = Triangle::new(p0, p1, p2);

        assert_eps_eq(&triangle.p0, &p0, EPS);
        assert_eps_eq(&triangle.p1, &p1, EPS);
        assert_eps_eq(&triangle.p2, &p2, EPS);
        assert_eps_eq(&triangle.e0, &vector(-1.0, -1.0, 0.0), EPS);
        assert_eps_eq(&triangle.e1, &vector(1.0, -1.0, 0.0), EPS);
        assert_eps_eq(&triangle.normal, &vector(0.0, 0.0, -1.0), EPS);
    }

    #[test]
    fn test_triangle_normal() {
        let p0 = point(0.0, 1.0, 0.0);
        let p1 = point(-1.0, 0.0, 0.0);
        let p2 = point(1.0, 0.0, 0.0);
        let triangle = Triangle::new(p0, p1, p2);

        assert_eps_eq(
            &triangle.normal(point(0.0, 0.5, 0.0)),
            &triangle.normal,
            EPS,
        );
        assert_eps_eq(
            &triangle.normal(point(-0.5, 0.75, 0.0)),
            &triangle.normal,
            EPS,
        );
        assert_eps_eq(
            &triangle.normal(point(0.5, 0.25, 0.0)),
            &triangle.normal,
            EPS,
        );
    }

    #[test]
    fn test_triangle_intersect() {
        let p0 = point(0.0, 1.0, 0.0);
        let p1 = point(-1.0, 0.0, 0.0);
        let p2 = point(1.0, 0.0, 0.0);
        let triangle = Triangle::new(p0, p1, p2);

        let origin = point(0.0, -1.0, -2.0);
        let dir = vector(0.0, 1.0, 0.0);
        let ray = Ray::new(origin, dir);

        // ray is parallel to the triangle; no intersect
        assert_eq!(triangle.intersect(&ray), None);
    }
}
