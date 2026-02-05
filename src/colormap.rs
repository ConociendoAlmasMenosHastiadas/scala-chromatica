//! Color gradients with smooth interpolation
//!
//! A ColorMap consists of multiple ColorStops positioned along a gradient (0.0 to 1.0).
//! Colors between stops are computed using linear RGB interpolation.
//!
//! # Example
//! ```
//! use scala_chromatica::{ColorMap, ColorStop, Color};
//!
//! let mut map = ColorMap::new("RedToBlue");
//! map.add_stop(ColorStop::new(0.0, Color::new(255, 0, 0)));
//! map.add_stop(ColorStop::new(1.0, Color::new(0, 0, 255)));
//!
//! let mid_color = map.get_color(0.5); // Gets color halfway between red and blue
//! ```

use crate::color::Color;
use serde::{Deserialize, Serialize};

/// A color stop in a gradient (position + color)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ColorStop {
    /// Position along the gradient (0.0 to 1.0)
    pub position: f64,
    /// RGB color at this position
    pub color: Color,
    /// Optional name for documentation/UI purposes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl ColorStop {
    /// Create a new color stop
    pub fn new(position: f64, color: Color) -> Self {
        Self {
            position: position.clamp(0.0, 1.0),
            color,
            name: None,
        }
    }

    /// Create a new color stop with a name
    pub fn with_name(position: f64, color: Color, name: impl Into<String>) -> Self {
        Self {
            position: position.clamp(0.0, 1.0),
            color,
            name: Some(name.into()),
        }
    }
}

/// A colormap with multiple color stops and smooth interpolation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorMap {
    /// Name of the colormap
    pub name: String,
    /// Ordered list of color stops
    pub stops: Vec<ColorStop>,
}

impl ColorMap {
    /// Create a new colormap with a given name
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            stops: Vec::new(),
        }
    }

    /// Create a colormap with initial stops
    pub fn with_stops(name: impl Into<String>, stops: Vec<ColorStop>) -> Self {
        let mut colormap = Self {
            name: name.into(),
            stops,
        };
        colormap.sort_stops();
        colormap
    }

    /// Add a color stop to the gradient
    pub fn add_stop(&mut self, stop: ColorStop) {
        self.stops.push(stop);
        self.sort_stops();
    }

    /// Remove a color stop by index (minimum 2 stops required)
    pub fn remove_stop(&mut self, index: usize) {
        if index < self.stops.len() && self.stops.len() > 2 {
            self.stops.remove(index);
        }
    }

    /// Sort stops by position (maintains gradient order)
    fn sort_stops(&mut self) {
        self.stops
            .sort_by(|a, b| a.position.partial_cmp(&b.position).unwrap());
    }

    /// Get color at a specific position (0.0 to 1.0) by interpolating between stops
    pub fn get_color(&self, position: f64) -> Color {
        let position = position.clamp(0.0, 1.0);

        if self.stops.is_empty() {
            return Color::black();
        }

        if self.stops.len() == 1 {
            return self.stops[0].color;
        }

        // Before first stop
        if position <= self.stops[0].position {
            return self.stops[0].color;
        }

        // After last stop
        if position >= self.stops.last().unwrap().position {
            return self.stops.last().unwrap().color;
        }

        // Find surrounding stops and interpolate
        for i in 0..self.stops.len() - 1 {
            let stop1 = &self.stops[i];
            let stop2 = &self.stops[i + 1];

            if position >= stop1.position && position <= stop2.position {
                let range = stop2.position - stop1.position;
                let t = if range > 0.0 {
                    (position - stop1.position) / range
                } else {
                    0.0
                };
                return stop1.color.lerp(&stop2.color, t);
            }
        }

        // Fallback to last color
        self.stops.last().unwrap().color
    }

    /// Default HSV-based color scheme (smooth rainbow)
    pub fn default_scheme() -> Self {
        Self::with_stops(
            "Default",
            vec![
                ColorStop::new(0.0, Color::black()),
                ColorStop::new(0.2, Color::from_hsv(240.0, 1.0, 1.0)), // Blue
                ColorStop::new(0.5, Color::from_hsv(120.0, 1.0, 1.0)), // Green
                ColorStop::new(0.8, Color::from_hsv(0.0, 1.0, 1.0)),   // Red
                ColorStop::new(1.0, Color::white()),
            ],
        )
    }

    /// Fire color scheme (black -> red -> orange -> yellow -> white)
    pub fn fire_scheme() -> Self {
        Self::with_stops(
            "Fire",
            vec![
                ColorStop::new(0.0, Color::black()),
                ColorStop::new(0.25, Color::new(128, 0, 0)), // Dark red
                ColorStop::new(0.5, Color::new(255, 0, 0)),  // Red
                ColorStop::new(0.75, Color::new(255, 128, 0)), // Orange
                ColorStop::new(0.9, Color::new(255, 255, 0)), // Yellow
                ColorStop::new(1.0, Color::white()),
            ],
        )
    }

    /// Ocean color scheme (black -> deep blue -> cyan -> white)
    pub fn ocean_scheme() -> Self {
        Self::with_stops(
            "Ocean",
            vec![
                ColorStop::new(0.0, Color::black()),
                ColorStop::new(0.3, Color::new(0, 0, 128)),   // Deep blue
                ColorStop::new(0.6, Color::new(0, 128, 255)), // Sky blue
                ColorStop::new(0.85, Color::new(0, 255, 255)), // Cyan
                ColorStop::new(1.0, Color::white()),
            ],
        )
    }

    /// Grayscale color scheme (black -> gray -> white)
    pub fn grayscale_scheme() -> Self {
        Self::with_stops(
            "Grayscale",
            vec![
                ColorStop::new(0.0, Color::black()),
                ColorStop::new(0.5, Color::new(128, 128, 128)),
                ColorStop::new(1.0, Color::white()),
            ],
        )
    }

    /// Rainbow color scheme (full spectrum)
    pub fn rainbow_scheme() -> Self {
        Self::with_stops(
            "Rainbow",
            vec![
                ColorStop::new(0.0, Color::from_hsv(0.0, 1.0, 1.0)),   // Red
                ColorStop::new(0.17, Color::from_hsv(60.0, 1.0, 1.0)), // Yellow
                ColorStop::new(0.33, Color::from_hsv(120.0, 1.0, 1.0)), // Green
                ColorStop::new(0.5, Color::from_hsv(180.0, 1.0, 1.0)), // Cyan
                ColorStop::new(0.67, Color::from_hsv(240.0, 1.0, 1.0)), // Blue
                ColorStop::new(0.83, Color::from_hsv(300.0, 1.0, 1.0)), // Magenta
                ColorStop::new(1.0, Color::from_hsv(360.0, 1.0, 1.0)), // Red
            ],
        )
    }
}

/// Convert iteration count to color using a colormap
///
/// This is a utility function for fractal rendering and similar applications
/// where you need to map iteration counts to colors.
///
/// # Arguments
/// * `iterations` - Number of iterations performed
/// * `max_iterations` - Maximum iterations allowed
/// * `colormap` - The colormap to use for coloring
/// * `use_period` - Enable periodic color cycling
/// * `period` - Period for color cycling (if enabled)
/// * `use_interior_color` - Use custom color for interior points
/// * `interior_color` - RGB color for interior points
/// * `use_log_scale` - Apply logarithmic scaling to colors
pub fn color_from_iterations(
    iterations: u32,
    max_iterations: u32,
    colormap: &ColorMap,
    use_period: bool,
    period: u32,
    use_interior_color: bool,
    interior_color: [u8; 3],
    use_log_scale: bool,
) -> Color {
    // Check if point is inside the set and custom interior color is enabled
    if iterations >= max_iterations && use_interior_color {
        return Color {
            r: interior_color[0],
            g: interior_color[1],
            b: interior_color[2],
        };
    }

    // Apply period modulation if enabled
    let effective_iterations = if use_period && period > 0 {
        iterations % period
    } else {
        iterations
    };

    // Normalize iterations to 0.0-1.0 range
    let divisor = if use_period && period > 0 {
        period as f64
    } else {
        max_iterations as f64
    };
    let t = effective_iterations as f64 / divisor;

    // Apply smooth coloring - use log scale if enabled, otherwise linear
    let smooth_t = if use_log_scale {
        (t * 10.0).log10() / 1.0 // log10(10) = 1
    } else {
        t // Linear scaling
    };

    colormap.get_color(smooth_t.clamp(0.0, 1.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_colorstop_creation() {
        let stop = ColorStop::new(0.5, Color::new(255, 0, 0));
        assert_eq!(stop.position, 0.5);
        assert_eq!(stop.color.r, 255);
        assert!(stop.name.is_none());

        let named_stop = ColorStop::with_name(0.3, Color::new(0, 255, 0), "Green");
        assert_eq!(named_stop.name, Some("Green".to_string()));
    }

    #[test]
    fn test_colormap_gradient() {
        let mut map = ColorMap::new("Test");
        map.add_stop(ColorStop::new(0.0, Color::new(0, 0, 0)));
        map.add_stop(ColorStop::new(1.0, Color::new(255, 255, 255)));

        let start = map.get_color(0.0);
        assert_eq!(start.r, 0);

        let end = map.get_color(1.0);
        assert_eq!(end.r, 255);

        let mid = map.get_color(0.5);
        assert!(mid.r > 100 && mid.r < 200);
    }

    #[test]
    fn test_builtin_schemes() {
        let default = ColorMap::default_scheme();
        assert_eq!(default.name, "Default");
        assert!(!default.stops.is_empty());

        let fire = ColorMap::fire_scheme();
        assert_eq!(fire.name, "Fire");

        let ocean = ColorMap::ocean_scheme();
        assert_eq!(ocean.name, "Ocean");

        let grayscale = ColorMap::grayscale_scheme();
        assert_eq!(grayscale.name, "Grayscale");

        let rainbow = ColorMap::rainbow_scheme();
        assert_eq!(rainbow.name, "Rainbow");
    }
}
