//! RGB Color with HSV conversion and interpolation
//!
//! Provides a simple RGB color representation with support for:
//! - RGB color creation
//! - HSV to RGB conversion
//! - Linear interpolation (lerp) between colors
//! - Common color constants (black, white)

use serde::{Deserialize, Serialize};

/// RGB Color representation
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    /// Create a new RGB color
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Create a color from HSV values
    ///
    /// # Arguments
    /// * `h` - Hue (0.0 - 360.0)
    /// * `s` - Saturation (0.0 - 1.0)
    /// * `v` - Value/Brightness (0.0 - 1.0)
    pub fn from_hsv(h: f64, s: f64, v: f64) -> Self {
        let c = v * s;
        let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
        let m = v - c;

        let (r, g, b) = if h < 60.0 {
            (c, x, 0.0)
        } else if h < 120.0 {
            (x, c, 0.0)
        } else if h < 180.0 {
            (0.0, c, x)
        } else if h < 240.0 {
            (0.0, x, c)
        } else if h < 300.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };

        Self {
            r: ((r + m) * 255.0) as u8,
            g: ((g + m) * 255.0) as u8,
            b: ((b + m) * 255.0) as u8,
        }
    }

    /// Pure black color (0, 0, 0)
    pub fn black() -> Self {
        Self::new(0, 0, 0)
    }

    /// Pure white color (255, 255, 255)
    pub fn white() -> Self {
        Self::new(255, 255, 255)
    }

    /// Linear interpolation between two colors
    ///
    /// # Arguments
    /// * `other` - The target color to interpolate towards
    /// * `t` - Interpolation factor (0.0 = self, 1.0 = other)
    pub fn lerp(&self, other: &Color, t: f64) -> Color {
        let t = t.clamp(0.0, 1.0);
        Color {
            r: (self.r as f64 + (other.r as f64 - self.r as f64) * t) as u8,
            g: (self.g as f64 + (other.g as f64 - self.g as f64) * t) as u8,
            b: (self.b as f64 + (other.b as f64 - self.b as f64) * t) as u8,
        }
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RGB({},{},{})", self.r, self.g, self.b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_creation() {
        let color = Color::new(255, 128, 64);
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 128);
        assert_eq!(color.b, 64);
    }

    #[test]
    fn test_color_constants() {
        let black = Color::black();
        assert_eq!(black.r, 0);
        assert_eq!(black.g, 0);
        assert_eq!(black.b, 0);

        let white = Color::white();
        assert_eq!(white.r, 255);
        assert_eq!(white.g, 255);
        assert_eq!(white.b, 255);
    }

    #[test]
    fn test_lerp() {
        let red = Color::new(255, 0, 0);
        let blue = Color::new(0, 0, 255);

        let mid = red.lerp(&blue, 0.5);
        assert_eq!(mid.r, 127);
        assert_eq!(mid.g, 0);
        assert_eq!(mid.b, 127);

        let at_red = red.lerp(&blue, 0.0);
        assert_eq!(at_red.r, 255);

        let at_blue = red.lerp(&blue, 1.0);
        assert_eq!(at_blue.b, 255);
    }

    #[test]
    fn test_hsv_conversion() {
        // Pure red (H=0)
        let red = Color::from_hsv(0.0, 1.0, 1.0);
        assert_eq!(red.r, 255);
        assert_eq!(red.g, 0);
        assert_eq!(red.b, 0);

        // Pure green (H=120)
        let green = Color::from_hsv(120.0, 1.0, 1.0);
        assert_eq!(green.r, 0);
        assert_eq!(green.g, 255);
        assert_eq!(green.b, 0);

        // Pure blue (H=240)
        let blue = Color::from_hsv(240.0, 1.0, 1.0);
        assert_eq!(blue.r, 0);
        assert_eq!(blue.g, 0);
        assert_eq!(blue.b, 255);
    }
}
