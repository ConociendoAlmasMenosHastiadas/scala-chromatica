# Changelog

All notable changes to scala-chromatica will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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

[0.1.0]: https://github.com/ConociendoAlmasMenosHastiadas/scala-chromatica/releases/tag/v0.1.0
