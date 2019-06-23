use std::ops::Mul;
use cgmath::{Angle, InnerSpace, Vector2, Vector3};
use cgmath::vec3;

/// A 3 x 2 column major matrix representing a pixel to camera ray transform.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CameraMatrix {
    /// The first column of the matrix.
    pub x: Vector3<f64>,
    /// The second column of the matrix.
    pub y: Vector3<f64>,
}

impl CameraMatrix {
    // Camera is positioned at (0, 0, 0) and faces in the negative z directon.
    // The "film" plane is parallel to the XY plane and is 1 unit away from the
    // origin along the negative Z axis.
    pub fn camera(
        fov: impl Angle<Unitless = f64>,
        x_max: u32,
        y_max: u32,
        x: u32,
        y: u32,
    ) -> Vector3<f64> {
        // convert to pixel coordinates to normalized device coordinates pixel
        // coordinates start with the origin (0, 0) as the top left-most pixel
        // on the screen, and we compute normalized device coordinates by
        // transforming them to the range [0, 1]. We add 0.5 to get the ray
        // going through the center of the pixel.
        let x_ndc = ((x as f64) + 0.5) / (x_max as f64);
        let y_ndc = ((y as f64) + 0.5) / (y_max as f64);

        // Screen coordinates transform normalized device coordinates so that
        // (0, 0) is straight down the negative Z axis. Screen coordinates are
        // in the range [-1, 1].
        let x_screen = (2.0 * x_ndc) - 1.0;
        let y_screen = 1.0 - (2.0 * y_ndc);

        // Convert to camera coordinates:
        // Stretch out the [-1, 1] x [-1, 1] square according to the aspect
        // ratio. Use the field of view to determine how wide to shoot the rays.
        let fov = Angle::tan(fov / 2.0);
        let aspect_ratio = (x_max as f64) / (y_max as f64);
        let x_camera = (x_screen) * aspect_ratio * fov;
        let y_camera = y_screen * fov;

        // Shoot a ray at the pixel.
        vec3(x_camera, y_camera, -1.0)
    }

    /// Create a new camera matrix, providing a field of view and image
    /// dimensions.
    pub fn new(
        fov: impl Angle<Unitless = f64>,
        x_max: u32,
        y_max: u32,
    ) -> CameraMatrix {
        let c0 = CameraMatrix::camera(fov, x_max, y_max, 1, 0);
        let c1 = CameraMatrix::camera(fov, x_max, y_max, 0, 1);

        CameraMatrix::from_cols(c0, c1)
    }

    /// Create a new camera matrix, providing columns.
    #[inline]
    pub const fn from_cols(c0: Vector3<f64>, c1: Vector3<f64>) -> CameraMatrix {
        CameraMatrix { x: c0, y: c1 }
    }
}

impl Mul<Vector2<f64>> for CameraMatrix {
    type Output = Vector3<f64>;

    fn mul(self, rhs: Vector2<f64>) -> Vector3<f64> {
        // TODO: could be fun to optimize this with simd
        // cgmath already does this for Vector4 using the "simd" crate
        self.x * rhs[0] + self.y * rhs[1]
    }
}
