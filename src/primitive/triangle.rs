use super::Scene;
use super::math::{Ray, vector};

use ultraviolet::vec::{DVec3, DVec4};

pub struct Triangle {
    p1: DVec4,
    p2: DVec4,
    p3: DVec4,
    e1: DVec4,
    e2: DVec4,
    normal: DVec3,
}

impl Triangle {
    pub fn new(p1: DVec4, p2: DVec4, p3: DVec4) -> Triangle {
        // compute edge vectors
        let e1 = p2 - p1;
        let e2 = p3 - p1;

        // compute normal vector
        let mut normal = e2.xyz().cross(e1.xyz());
        normal.normalize();

        Triangle {
            p1,
            p2,
            p3,
            e1,
            e2,
            normal,
        }
    }

    pub fn normal(&self) -> DVec3 {
        self.normal
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
    fn test_triangle_normal() {
        let p1 = point(0.0, 1.0, 0.0);
        let p2 = point(-1.0, 0.0, 0.0);
        let p3 = point(1.0, 0.0, 0.0);
        let triangle = Triangle::new(p1, p2, p3);

        assert_eps_eq(&triangle.p1, &p1, EPS);
        assert_eps_eq(&triangle.p2, &p2, EPS);
        assert_eps_eq(&triangle.p3, &p3, EPS);
        assert_eps_eq(&triangle.e1, &vector(-1.0, -1.0, 0.0), EPS);
        assert_eps_eq(&triangle.e2, &vector(1.0, -1.0, 0.0), EPS);
        assert_eps_eq(&triangle.normal, &DVec3::new(0.0, 0.0, -1.0), EPS);
    }
}
