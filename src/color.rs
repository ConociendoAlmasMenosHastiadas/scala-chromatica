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

    /// Parse a hex color string into a Color
    ///
    /// Supports the following formats:
    /// - `#RGB` (e.g., `#F0A`)
    /// - `#RRGGBB` (e.g., `#FF00AA`)
    /// - `RGB` (without #)
    /// - `RRGGBB` (without #)
    ///
    /// # Examples
    /// ```
    /// use scala_chromatica::Color;
    ///
    /// let color1 = Color::from_hex("#FF5733").unwrap();
    /// assert_eq!(color1.r, 255);
    /// assert_eq!(color1.g, 87);
    /// assert_eq!(color1.b, 51);
    ///
    /// let color2 = Color::from_hex("#F0A").unwrap();
    /// assert_eq!(color2.r, 255);
    /// assert_eq!(color2.g, 0);
    /// assert_eq!(color2.b, 170);
    /// ```
    pub fn from_hex(hex: &str) -> crate::error::Result<Self> {
        let hex = hex.trim().trim_start_matches('#');
        
        match hex.len() {
            3 => {
                // RGB format - expand each digit
                let r = u8::from_str_radix(&hex[0..1].repeat(2), 16)
                    .map_err(|_| crate::error::ColorMapError::InvalidHexColor(hex.to_string()))?;
                let g = u8::from_str_radix(&hex[1..2].repeat(2), 16)
                    .map_err(|_| crate::error::ColorMapError::InvalidHexColor(hex.to_string()))?;
                let b = u8::from_str_radix(&hex[2..3].repeat(2), 16)
                    .map_err(|_| crate::error::ColorMapError::InvalidHexColor(hex.to_string()))?;
                Ok(Self::new(r, g, b))
            }
            6 => {
                // RRGGBB format
                let r = u8::from_str_radix(&hex[0..2], 16)
                    .map_err(|_| crate::error::ColorMapError::InvalidHexColor(hex.to_string()))?;
                let g = u8::from_str_radix(&hex[2..4], 16)
                    .map_err(|_| crate::error::ColorMapError::InvalidHexColor(hex.to_string()))?;
                let b = u8::from_str_radix(&hex[4..6], 16)
                    .map_err(|_| crate::error::ColorMapError::InvalidHexColor(hex.to_string()))?;
                Ok(Self::new(r, g, b))
            }
            _ => Err(crate::error::ColorMapError::InvalidHexColor(hex.to_string())),
        }
    }

    /// Convert a Color to a hex string (e.g., "#FF5733")
    ///
    /// # Examples
    /// ```
    /// use scala_chromatica::Color;
    ///
    /// let color = Color::new(255, 87, 51);
    /// assert_eq!(color.to_hex(), "#FF5733");
    /// ```
    pub fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
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

    #[test]
    fn test_from_hex() {
        // Test #RRGGBB format
        let color1 = Color::from_hex("#FF5733").unwrap();
        assert_eq!(color1.r, 255);
        assert_eq!(color1.g, 87);
        assert_eq!(color1.b, 51);

        // Test RRGGBB format without #
        let color2 = Color::from_hex("00FF00").unwrap();
        assert_eq!(color2.r, 0);
        assert_eq!(color2.g, 255);
        assert_eq!(color2.b, 0);

        // Test #RGB format
        let color3 = Color::from_hex("#F0A").unwrap();
        assert_eq!(color3.r, 255);
        assert_eq!(color3.g, 0);
        assert_eq!(color3.b, 170);

        // Test RGB format without #
        let color4 = Color::from_hex("C8F").unwrap();
        assert_eq!(color4.r, 204);
        assert_eq!(color4.g, 136);
        assert_eq!(color4.b, 255);

        // Test with whitespace
        let color5 = Color::from_hex("  #ABCDEF  ").unwrap();
        assert_eq!(color5.r, 171);
        assert_eq!(color5.g, 205);
        assert_eq!(color5.b, 239);

        // Test invalid formats
        assert!(Color::from_hex("#GGGGGG").is_err());
        assert!(Color::from_hex("#12345").is_err());
        assert!(Color::from_hex("").is_err());
    }

    #[test]
    fn test_to_hex() {
        let color1 = Color::new(255, 87, 51);
        assert_eq!(color1.to_hex(), "#FF5733");

        let color2 = Color::new(0, 255, 0);
        assert_eq!(color2.to_hex(), "#00FF00");

        let color3 = Color::new(255, 0, 170);
        assert_eq!(color3.to_hex(), "#FF00AA");
    }

    #[test]
    fn test_hex_roundtrip() {
        let original = Color::new(123, 45, 67);
        let hex = original.to_hex();
        let parsed = Color::from_hex(&hex).unwrap();
        assert_eq!(original, parsed);
    }
}
