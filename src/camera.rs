#![allow(dead_code)]

use cgmath::{Angle, Matrix3, Vector3, vec3};

/// Generate primary rays for the pixel (x, y) from a camera positioned at
/// (0, 0, 0) that faces in the negative z directon. The "film" plane is
/// parallel to the XY plane and is 1 unit away from the origin along the
/// negative Z axis.
pub fn projection_function(
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
    let x_ndc = (x as f64 + 0.5) / x_max as f64;
    let y_ndc = (y as f64 + 0.5) / y_max as f64;

    // Screen coordinates transform normalized device coordinates so that
    // (0, 0) is straight down the negative Z axis. Screen coordinates are
    // in the range [-1, 1].
    let x_screen = (2.0 * x_ndc) - 1.0;
    let y_screen = 1.0 - (2.0 * y_ndc);

    // Convert to camera coordinates:
    // Stretch out the [-1, 1] x [-1, 1] square according to the aspect
    // ratio. Use the field of view to determine how wide to shoot the rays.
    let fov = Angle::tan(fov / 2.0);
    let aspect_ratio = x_max as f64 / y_max as f64;
    let x_camera = (x_screen) * aspect_ratio * fov;
    let y_camera = y_screen * fov;

    // Shoot a ray at the pixel.
    vec3(x_camera, y_camera, -1.0)
}

/// Create a 3x3 camera matrix, which transforms pixel coordinates of the vector
/// form (x_pixel y_pixel 1.0)^T into primary rays. This matrix is the
/// `projection_function` condensed and simplified into an augmented matrix.
pub fn projection_matrix(
    fov: impl Angle<Unitless = f64>,
    x_max: u32,
    y_max: u32,
) -> Matrix3<f64> {
    let fov = Angle::tan(fov / 2.0);
    let x_max = x_max as f64;
    let y_max = y_max as f64;

    let coef_x = 2.0 * fov / y_max;
    let offs_x = (1.0 - x_max) * fov / y_max;

    let coef_y = -2.0 * fov / y_max;
    let offs_y = (y_max - 1.0) * fov / y_max;

    let c0 = vec3(coef_x, 0.0, 0.0); // scaled by pixel_x
    let c1 = vec3(0.0, coef_y, 0.0); // scaled by pixel_y
    let c2 = vec3(offs_x, offs_y, -1.0); // added to pixel_x, pixel_y, sets z
    Matrix3::from_cols(c0, c1, c2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cgmath::{Deg, assert_abs_diff_eq};

    #[test]
    fn test_projection_matrix() {
        let fov = Deg(100.0);
        let x_max = 1600;
        let y_max = 900;
        let matrix = projection_matrix(fov, x_max, y_max);

        for pixel_x in 0..x_max {
            for pixel_y in 0..y_max {
                assert_abs_diff_eq!(
                    projection_function(fov, x_max, y_max, pixel_x, pixel_y),
                    matrix * vec3(pixel_x as f64, pixel_y as f64, 1.0),
                    epsilon = 1.0e-5,
                );
            }
        }
    }
}
