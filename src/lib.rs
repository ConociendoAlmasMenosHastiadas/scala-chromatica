//! scala-chromatica: A framework-agnostic color gradient library
//!
//! This library provides smooth color interpolation, gradient management,
//! and persistent storage for color schemes. Perfect for data visualization,
//! fractal rendering, scientific plots, and any application requiring
//! sophisticated color mapping.
//!
//! # Features
//! - Smooth RGB interpolation between color stops
//! - HSV color space support
//! - JSON serialization/deserialization
//! - 15 built-in color schemes (Fire, Ocean, Rainbow, etc.)
//! - Platform-specific config directory management
//! - Custom colormap save/load
//!
//! # Quick Start
//! ```rust
//! use scala_chromatica::{ColorMap, Color, ColorStop};
//!
//! // Load a built-in colormap
//! let fire = scala_chromatica::io::load_builtin_colormap("Fire").unwrap();
//!
//! // Get color at 50% position
//! let color = fire.get_color(0.5);
//! println!("RGB: ({}, {}, {})", color.r, color.g, color.b);
//!
//! // Create a custom gradient
//! let mut custom = ColorMap::new("Custom Gradient");
//! custom.add_stop(ColorStop::new(0.0, Color::new(255, 0, 0)));
//! custom.add_stop(ColorStop::new(1.0, Color::new(0, 0, 255)));
//!
//! // Save for later use
//! scala_chromatica::io::save_colormap(&custom).unwrap();
//! ```

pub mod color;
pub mod colormap;
pub mod error;
pub mod io;

// Re-export main types at crate root for convenience
pub use color::Color;
pub use colormap::{color_from_iterations, ColorMap, ColorStop};
pub use error::{ColorMapError, Result};
