# scala-chromatica

A framework-agnostic Rust library for color gradients with smooth interpolation.

[![Crates.io](https://img.shields.io/crates/v/scala-chromatica.svg)](https://crates.io/crates/scala-chromatica)
[![Documentation](https://docs.rs/scala-chromatica/badge.svg)](https://docs.rs/scala-chromatica)
[![License](https://img.shields.io/crates/l/scala-chromatica.svg)](https://github.com/ConociendoAlmasMenosHastiadas/scala-chromatica#license)

## Features

- 🎨 **Smooth RGB interpolation** between color stops
- 🌈 **HSV color space support** for vibrant gradients
- 💾 **JSON serialization** for persistent storage
- 📦 **14 built-in color schemes** embedded at compile time
- 📁 **Platform-specific config directories** for custom colormaps
- 🚫 **No GUI framework dependencies** - use with any rendering system

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
scala-chromatica = "0.1.0"
```

## Quick Example

```rust
use scala_chromatica::{ColorMap, Color, ColorStop};

// Load a built-in colormap
let fire = scala_chromatica::io::load_builtin_colormap("Fire")?;
let color = fire.get_color(0.5);
println!("RGB: ({}, {}, {})", color.r, color.g, color.b);

// Create a custom gradient
let mut custom = ColorMap::new("RedToBlue");
custom.add_stop(ColorStop::new(0.0, Color::new(255, 0, 0)));
custom.add_stop(ColorStop::new(1.0, Color::new(0, 0, 255)));

// Save for later
scala_chromatica::io::save_colormap(&custom)?;
```

## Built-in Colormaps

**Basic:** Default, Fire, Ocean, Grayscale, Rainbow

**Extended:** Academic, Twilight Garden, Coral Sunset, Olive Symmetry, Orchid Garden, Frozen Amaranth, Electric Neon, Cosmic Dawn, Vintage Lavender

## Use Cases

- 📊 Data visualization and scientific plotting
- 🖼️ Fractal rendering (Mandelbrot, Julia sets)
- 🎮 Game development (heatmaps, effects)
- 🗺️ Geographic data visualization
- 📈 Any application requiring smooth color transitions

## Documentation

Full API documentation is available on [docs.rs](https://docs.rs/scala-chromatica).

## License

Dually licensed under MIT OR Apache-2.0 at your option.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Origin

Extracted from [forma-fractalis](https://github.com/ConociendoAlmasMenosHastiadas/forma-fractalis), a high-performance fractal renderer, to enable reuse across other visualization projects.
