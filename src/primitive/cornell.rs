use super::{Scene, Ray};
use cgmath::{InnerSpace, Vector3, vec2, vec3};

/// Representation of a parallelogram in 3D space.
struct Parallelogram {
    p: Vector3<f64>,
    u: Vector3<f64>,
    v: Vector3<f64>,
    normal: Vector3<f64>,
    color: Vector3<u8>,
    emittance: Option<Vector3<u8>>,
}

impl Parallelogram {
    fn new(
        p: Vector3<f64>,
        u: Vector3<f64>,
        v: Vector3<f64>,
        color: Vector3<u8>,
    ) -> Parallelogram {
        Parallelogram {
            p, u, v, color,
            normal: u.cross(v).normalize(),
            emittance: None,
        }
    }

    fn set_emissive(&mut self, color: Vector3<u8>) {
        self.emittance = Some(color);
    }

    fn intersect(&self, ray: &Ray) -> Option<Ray> {
        let denom = ray.direction.dot(self.normal);
        if denom.abs() > 1.0e-4 {
            let t = (self.p - ray.origin).dot(self.normal) / denom;
            if t > 1.0e-4 {
                let intersect = ray.origin + ray.direction * t;
                if self.check_bounds(intersect) {
                    return Some(Ray::new(intersect, self.normal.clone()));
                }
            }
        }

        // TODO: borrow normals from the surface instead of cloning them?
        None
    }

    fn check_bounds(&self, point: Vector3<f64>) -> bool {
        // express the point as components of u and v
        let point = point - self.p;
        let point = vec2(point.dot(self.u), point.dot(self.v));

        // in this coordinate system the parallelogram is bounded by
        // the square (0, 0), (0, 1), (1, 1), (1, 0)
        !(point.y < 0.0 || point.y > 1.0 || point.x < 0.0 || point.x > 1.0)
    }
}

pub struct CornellBox {
    surfaces: Vec<Parallelogram>,
}

impl CornellBox {
    pub fn new() -> CornellBox {
        let white = vec3(255, 255, 255);
        let green = vec3(0, 255, 0);
        let red = vec3(255, 0, 0);
        let mut surfaces = Vec::new();

        let test_a = Parallelogram::new(
            vec3(0.0, 0.0, -2.0),
            vec3(0.0, 1.0, 0.0),
            vec3(1.0, 0.0, 0.0),
            white,
        );

        let test_b = Parallelogram::new(
            vec3(-1.0, -1.0, -2.0),
            vec3(0.0, 1.0, 0.0),
            vec3(1.0, 0.0, 0.0),
            white,
        );

        surfaces.push(test_a);
        surfaces.push(test_b);

        CornellBox { surfaces }
    }
}

impl Scene for CornellBox {
    fn intersect(&self, ray: &Ray) -> Option<Ray> {
        let mut min_dist = None;
        let mut min_normal = None;

        for surface in &self.surfaces {
            if let Some(normal) = surface.intersect(ray) {
                // avoid sqrt
                let dist = (normal.origin - ray.origin).magnitude2();
                min_dist = match min_dist {
                    None => {
                        min_normal = Some(normal);
                        Some(dist)
                    },
                    Some(cur_min) => {
                        if dist < cur_min {
                            min_normal = Some(normal);
                            Some(dist)
                        } else {
                            Some(cur_min)
                        }
                    }
                }
            }
        }

        min_normal
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cgmath::assert_abs_diff_eq;

    fn make_test_surface() -> Parallelogram {
        let color = vec3(255, 255, 255);
        Parallelogram::new(
            vec3(0.0, 0.0, 0.0),
            vec3(2.0, 0.0, 0.0),
            vec3(0.0, 2.0, 0.0),
            color,
        )
    }

    #[test]
    fn test_surface_intersection() {
        let surface = make_test_surface();

        let ray_origin = vec3(1.0, 1.0, 10.0);
        let ray_direction = vec3(0.0, 0.0, -1.0);
        let ray = Ray::new(ray_origin, ray_direction);

        let intersection = surface.intersect(&ray).unwrap();
        assert_eq!(intersection.origin, vec3(1.0, 1.0, 0.0));
        assert_eq!(intersection.direction, vec3(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_surface_bounds() {
        let surface = make_test_surface();

        let ray_origin = vec3(1.0, 1.0, 10.0);
        let ray_direction = vec3(3.0, 0.0, -10.0);
        let ray = Ray::new(ray_origin, ray_direction);

        assert_eq!(surface.intersect(&ray), None);
    }
}
