#![allow(dead_code)]

use super::math::vector;
use ultraviolet::mat::DMat4;
use ultraviolet::vec::DVec4;

/// Generate primary rays for the pixel (x, y) from a camera positioned at
/// (0, 0, 0) that faces in the negative z directon. The "film" plane is
/// parallel to the XY plane and is 1 unit away from the origin along the
/// negative Z axis.
pub fn projection_function(
    fov: f64, // in radians
    x_max: u32,
    y_max: u32,
    x: u32,
    y: u32,
) -> DVec4 {
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
    let fov = f64::tan(fov / 2.0);
    let aspect_ratio = x_max as f64 / y_max as f64;
    let x_camera = (x_screen) * aspect_ratio * fov;
    let y_camera = y_screen * fov;

    // Shoot a ray at the pixel.
    vector(x_camera, y_camera, -1.0)
}

/// Create a 3x3 camera matrix, which transforms pixel coordinates of the vector
/// form (x_pixel y_pixel 1.0)^T into primary rays. This matrix is the
/// `projection_function` condensed and simplified into an augmented matrix.
pub fn projection_matrix(fov: f64, x_max: u32, y_max: u32) -> DMat4 {
    let fov = f64::tan(fov / 2.0);
    let x_max = x_max as f64;
    let y_max = y_max as f64;

    let coef_x = 2.0 * fov / y_max;
    let offs_x = (1.0 - x_max) * fov / y_max;

    let coef_y = -2.0 * fov / y_max;
    let offs_y = (y_max - 1.0) * fov / y_max;

    let c0 = vector(coef_x, 0.0, 0.0); // scaled by pixel_x
    let c1 = vector(0.0, coef_y, 0.0); // scaled by pixel_y
    let c2 = vector(offs_x, offs_y, -1.0); // added to pixel_x, pixel_y, sets z
    let c3 = vector(0.0, 0.0, 0.0);
    DMat4::new(c0, c1, c2, c3)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_projection_matrix() {
        let fov = f64::to_radians(100.0);
        let x_max = 1600;
        let y_max = 900;
        let matrix = projection_matrix(fov, x_max, y_max);

        for pixel_x in 0..x_max {
            for pixel_y in 0..y_max {
                let p_fn =
                    projection_function(fov, x_max, y_max, pixel_x, pixel_y);
                let p_mat =
                    matrix * vector(pixel_x as f64, pixel_y as f64, 1.0);
                let eps = 1.0e-6;
                assert!((p_fn.x - eps < p_mat.x) && (p_mat.x < p_fn.x + eps));
                assert!((p_fn.y - eps < p_mat.y) && (p_mat.y < p_fn.y + eps));
                assert!((p_fn.z - eps < p_mat.z) && (p_mat.z < p_fn.z + eps));
            }
        }
    }
}
