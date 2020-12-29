use ultraviolet::DVec3;
use std::ops::{Add, Sub, Mul};
use std::convert::From;

#[derive(Debug, PartialEq)]
struct Color(DVec3);

impl Color {
    fn new(r: f64, g: f64, b: f64) -> Self {
        Color(DVec3::new(r, g, b))
    }
}

impl From<DVec3> for Color {
    fn from(vec: DVec3) -> Self {
        Color(vec)
    }
}

impl Add<Self> for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Color(self.0 + rhs.0)
    }
}

impl Sub<Self> for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Color(self.0 - rhs.0)
    }
}

impl Mul<Self> for Color {
    type Output = Color;

    fn mul(self, rhs: Self) -> Self::Output {
        Color(self.0 * rhs.0)
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        Color(self.0 * rhs)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn assert_eps_eq(c1: Color, c2: Color) {
        let eps = 0.01;
        assert!((c2.0 - c1.0).abs().component_max() < eps)
    }

    #[test]
    fn test_color_add() {
        let a = Color::new(0.9, 0.6, 0.75);
        let b = Color::new(0.7, 0.1, 0.25);
        assert_eps_eq(a + b, Color::new(1.6, 0.7, 1.0));
    }

    #[test]
    fn test_color_sub() {
        let a = Color::new(0.9, 0.6, 0.75);
        let b = Color::new(0.7, 0.1, 0.25);
        assert_eps_eq(a - b, Color::new(0.2, 0.5, 0.5));
    }

    #[test]
    fn test_color_mul() {
        let a = Color::new(1.0, 0.2, 0.4);
        let b = Color::new(0.9, 1.0, 0.1);
        assert_eps_eq(a * b, Color::new(0.9, 0.2, 0.041));
    }

    #[test]
    fn test_color_mul_scalar() {
        let a = Color::new(0.2, 0.3, 0.4);
        let b = 2.0;
        assert_eps_eq(a * b, Color::new(0.4, 0.6, 0.8));
    }
}
