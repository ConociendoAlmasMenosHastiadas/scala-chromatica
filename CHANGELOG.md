# Changelog

All notable changes to scala-chromatica will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.3] - 2026-02-27

### Added
- New "Copper Sheen" colormap with metallic teal-to-copper gradient
- `Color::from_hex()` method for parsing hex color strings (#RGB, #RRGGBB formats)
- `Color::to_hex()` method for converting colors to hex strings
- `ColorMap::reversed()` method for reversing gradient direction
- Built-in colormap count increased from 16 to 17

### Changed
- Updated colormap_showcase example to include Copper Sheen
- Added comprehensive tests for hex color parsing and colormap reversal

## [0.1.2] - 2026-02-27

### Added
- New "Egyptian Echo" colormap with blue-to-yellow gradient inspired by ancient Egyptian art
- Built-in colormap count increased from 15 to 16

### Changed
- Updated README.md to reflect new colormap count and complete list
- Updated colormap_showcase example to include Egyptian Echo

## [0.1.1] - 2026-02-15

### Added
- New "Spring Meadow" colormap with earthy green tones
- `colormap_showcase` example with egui GUI for visualizing all colormaps
- Visual stop indicators in showcase to highlight interpolation anchor points
- Smooth mesh-based gradient rendering without visible artifacts

### Changed
- Updated documentation example to use "Custom Gradient" instead of "MyGradient"
- Simplified CI/CD to test only on Windows with stable Rust (KISS principle)
- Built-in colormap count increased from 14 to 15
- Showcase example now uses egui mesh API for superior visual quality

### Fixed
- Added `#[allow(clippy::too_many_arguments)]` to `color_from_iterations` function
- Improved gradient rendering to eliminate vertical line artifacts

## [0.1.0] - 2026-02-05

### Added
- Initial release of scala-chromatica
- Core `Color` type with RGB and HSV support
- `ColorMap` with smooth gradient interpolation between stops
- `ColorStop` positioning system for gradient definition
- 14 built-in color schemes (Default, Fire, Ocean, Rainbow, Academic, etc.)
- JSON serialization/deserialization via serde
- Platform-specific config directory support (Windows, Linux, macOS)
- Custom colormap save/load functionality
- `color_from_iterations()` helper for fractal rendering
- Comprehensive unit tests and documentation
- Example code in `examples/` directory

### Documentation
- Complete API documentation with examples
- README with quick start guide
- Dual MIT OR Apache-2.0 licensing

[0.1.3]: https://github.com/ConociendoAlmasMenosHastiadas/scala-chromatica/releases/tag/v0.1.3
[0.1.2]: https://github.com/ConociendoAlmasMenosHastiadas/scala-chromatica/releases/tag/v0.1.2
[0.1.1]: https://github.com/ConociendoAlmasMenosHastiadas/scala-chromatica/releases/tag/v0.1.1
[0.1.0]: https://github.com/ConociendoAlmasMenosHastiadas/scala-chromatica/releases/tag/v0.1.0
