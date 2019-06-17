extern crate cgmath;
use std::fs::File;
use std::io::Write;
use cgmath::{Deg, InnerSpace, Vector3, Vector4, vec2, vec3, vec4};

mod camera;
use camera::CameraMatrix;

struct Ray {
    origin: Vector3<f64>,
    direction: Vector3<f64>,
}

impl Ray {
    pub fn new(origin: Vector3<f64>, direction: Vector3<f64>) -> Ray {
        Ray {
            origin,
            direction: direction.normalize(),
        }
    }
}

struct Sphere {
    center: Vector3<f64>,
    radius: f64,
}

impl Sphere {
    // Returns the distance to the first intersection with a sphere, if it
    // exists.
    fn intersect(&self, ray: Ray) -> Option<f64> {
        let tmp = ray.origin - self.center;
        let b = 2.0 * tmp.dot(ray.direction);
        let c = tmp.dot(tmp) - self.radius.powi(2);

        let discriminant = b.powi(2) - 4.0 * c;

        if discriminant >= 0.0 {
            let discriminant = discriminant.sqrt();
            let solution_0 = -b - discriminant;
            let solution_1 = -b + discriminant;
            if solution_0 >= 0.0 {
                return Some(solution_0 / 2.0);
            } else if solution_1 >= 0.0 {
                return Some(solution_1 / 2.0);
            }
        }
        None
    }

    fn normal(&self, point: Vector3<f64>) -> Vector3<f64> {
        let x = (point.x - self.center.x) / self.radius;
        let y = (point.y - self.center.y) / self.radius;
        let z = (point.z - self.center.z) / self.radius;

        vec3(x, y, z)
    }
}

// bad for cache locality
#[derive(Debug, Copy, Clone)]
struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

struct ImageData {
    height: usize,
    width: usize,
    max_val: u16,
    buf: Box<[Pixel]>,
}

impl ImageData {
    // kind of a silly way to save image files for testing.
    // write throughput is an issue here since we're making very granular writes
    // and waiting for them to complete.
    //
    // TODO:
    // how does rust do write buffering? do I need to try something smarter?
    fn save_ppm(&self, filename: &str) -> std::io::Result<()> {
        let mut file = File::create(filename)?;

        // write ppm header
        write!(
            file,
            "P3\n{} {}\n{}\n",
            self.width, self.height, self.max_val
        )?;

        // write image data
        for row in 0..self.height {
            for col in 0..self.width {
                let pix = &self.buf[row * self.width + col];
                write!(file, "{} {} {}\t", pix.r, pix.g, pix.b)?;
            }
            write!(file, "\n")?;
        }

        Ok(())
    }
}

fn main() {
    const IMAGE_WIDTH: usize = 200;
    const IMAGE_HEIGHT: usize = 150;

    // alloate a row-major buffer
    // use std::vec? this overflows the stack for large arrays
    let buf =
        Box::new([Pixel { r: 0, g: 0, b: 0 }; IMAGE_WIDTH * IMAGE_HEIGHT]);

    let mut img = ImageData {
        height: IMAGE_HEIGHT,
        width: IMAGE_WIDTH,
        max_val: 256,
        buf: buf,
    };

    let fov = Deg(100.0);
    let cam = CameraMatrix::new(fov, IMAGE_WIDTH, IMAGE_HEIGHT);
    let origin = vec3(0.0, 0.0, 0.0);
    let sphere = Sphere {
        center: vec3(0.0, 0.0, -10.0),
        radius: 5.0,
    };

    for j in 0..IMAGE_HEIGHT {
        for i in 0..IMAGE_WIDTH {
            //let dir = cam * vec2(i as f64, j as f64);
            let dir =
                CameraMatrix::camera(fov, IMAGE_WIDTH, IMAGE_HEIGHT, i, j);
            // TODO: move this to a constructor; make sure we always normalize.
            // square roots are expensive though...
            //println!("{:#?}\t{} {}", dir.magnitude(), i, j);
            let ray = Ray {
                origin: origin,
                direction: dir.normalize(),
            };
            if let Some(_) = sphere.intersect(ray) {
                img.buf[j * IMAGE_WIDTH + i] = Pixel {
                    r: 255,
                    g: 255,
                    b: 255,
                };
            }
        }
    }

    img.save_ppm("render.ppm");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ray_sphere_intersect() {
        let sphere = Sphere {
            center: vec3(0.0, 0.0, 0.0),
            radius: 5.0,
        };

        // direction must be normalized otherwise we get weird intersect results
        let point = vec3(0.0, 0.0, 10.0);
        let direction = vec3(0.0, 0.0, -1.0);
        let ray = Ray::new(point, direction);

        let distance = sphere.intersect(ray);
        assert_eq!(distance.unwrap(), 5.0);
    }

    #[test]
    fn test_save_ppm() {
        const IMAGE_WIDTH: usize = 20;
        const IMAGE_HEIGHT: usize = 15;
        let buf = Box::new(
            [Pixel {
                r: 100,
                g: 100,
                b: 100,
            }; IMAGE_WIDTH * IMAGE_HEIGHT],
        );
        let mut img = ImageData {
            height: IMAGE_HEIGHT,
            width: IMAGE_WIDTH,
            max_val: 256,
            buf: buf,
        };

        img.save_ppm("test_save_ppm.ppm");
    }
}
