# scala-chromatica Agent Guidelines

## Overview

scala-chromatica is a Rust library for gradient-based color mapping. It provides built-in colormaps and utilities for creating custom gradients. This guide covers adding new colormaps and best practices for colormap development.

## Adding New Colormaps

### Using the Coolors Parser Utility

The easiest way to create new colormaps is using the Coolors.co parser utility:

1. **Create a palette on Coolors.co:**
   - Visit https://coolors.co/
   - Create or generate a color palette (5-7 colors recommended)
   - Export → XML format

2. **Save the XML file:**
   ```powershell
   # Save to build_scripts directory for organization
   cd scala-chromatica
   # Save your coolors.co XML as build_scripts/my_palette.xml
   ```

3. **Generate colormap JSON:**
   ```powershell
   python build_scripts/coolors_parser.py --pretty \
       -i build_scripts/my_palette.xml \
       -o src/colormaps/my_colormap.json
   ```

4. **Customize the colormap:**
   - Open `src/colormaps/my_colormap.json`
   - Edit the `"name"` field to be descriptive (e.g., "Ocean Depths", "Fire Storm")
   - Adjust positions if you want non-uniform distribution

5. **Register the colormap:**
   - Open `src/io.rs`
   - Find the `define_builtin_colormaps!` macro invocation
   - Add your colormap to the list:
     ```rust
     "My Colormap Name" => MY_COLORMAP_JSON => "colormaps/my_colormap.json",
     ```
   - The macro automatically handles all registration (loading, LUT generation, etc.)

6. **Test the colormap:**
   ```powershell
   cargo test
   cargo run --example basic_usage
   ```

### Manual Colormap Creation

If you prefer to create colormaps manually, create a JSON file in `src/colormaps/`:

```json
{
  "name": "Display Name",
  "stops": [
    {
      "position": 0.0,
      "color": {
        "r": 255,
        "g": 0,
        "b": 0
      }
    },
    {
      "position": 0.5,
      "color": {
        "r": 0,
        "g": 255,
        "b": 0
      }
    },
    {
      "position": 1.0,
      "color": {
        "r": 0,
        "g": 0,
        "b": 255
      }
    }
  ]
}
```

**Format requirements:**
- `position`: Float from 0.0 to 1.0, must be sorted in ascending order
- At least 2 gradient stops required
- RGB values: 0-255 (u8)
- Colors are linearly interpolated between stops

### HSV Gradient Creation

For more vibrant colormaps, consider using HSV color space. See `examples/hsv_gradients.rs` for techniques:

- **Rainbow gradients:** Rotate hue from 0° to 360°
- **Saturation effects:** High saturation for vibrant, low for pastel
- **Brightness variations:** Dark to bright transitions
- **Complementary pairs:** Opposite hues on color wheel

Example:
```rust
use scala_chromatica::{Color, ColorMap, ColorStop};

// Create a sunset gradient using HSV
let stops = vec![
    ColorStop::new(0.0, Color::from_hsv(0.0, 1.0, 0.5)),   // Deep red
    ColorStop::new(0.5, Color::from_hsv(30.0, 1.0, 1.0)),  // Orange
    ColorStop::new(1.0, Color::from_hsv(60.0, 0.8, 1.0)),  // Yellow
];
let sunset = ColorMap::new("HSV Sunset".to_string(), stops);
```

## Colormap Best Practices

### Naming Conventions
- Use descriptive names that evoke the color scheme
- Examples: "Ocean Depths", "Fire Storm", "Vintage Lavender"
- Avoid generic names like "Colormap1" or "Purple"

### Position Distribution
- **Smooth gradients:** Evenly distribute positions (0.0, 0.25, 0.5, 0.75, 1.0)
- **Sharp transitions:** Cluster positions close together
- **Emphasis:** Use more stops in areas you want to highlight

### Color Selection
- **High contrast:** Ensure visibility across different backgrounds
- **Perceptual uniformity:** Consider how colors appear to human eye
- **Colorblind-friendly:** Test with colorblindness simulators when possible
- **Avoid pure extremes:** (0,0,0) and (255,255,255) at stops can be harsh

### Testing Guidelines
- Test with different data ranges (0-100, 0-1000, negative values)
- Preview in target application (forma-fractalis for fractals)
- Check with different iteration counts if used for fractals
- Verify interpolation looks smooth at high resolutions

### Inspiration Sources
- Nature photography (sunsets, oceans, landscapes)
- Art and design (paintings, color theory)
- Scientific visualization (matplotlib, viridis)
- Paul Bourke's fractal gallery
- Color palette tools (Adobe Color, Coolors.co)

## Examples

The `examples/` directory contains demonstrations:

- **basic_usage.rs** - Loading built-in colormaps, mapping values, saving/loading
- **hsv_gradients.rs** - HSV color space techniques for vibrant gradients
- **custom_gradients.rs** - Advanced gradient manipulation, non-uniform stops, inversions

Run examples:
```powershell
cargo run --example basic_usage
cargo run --example hsv_gradients
cargo run --example custom_gradients
```

## Integration with forma-fractalis

When you add colormaps to scala-chromatica, they automatically become available in forma-fractalis:

1. **Update dependency:**
   ```powershell
   cd forma-fractalis
   cargo update -p scala-chromatica
   ```

2. **Rebuild forma-fractalis:**
   ```powershell
   cargo build --release
   ```

3. **New colormaps appear in GUI dropdown** - no code changes needed!

This separation allows scala-chromatica to be used by other projects beyond forma-fractalis.

## Development Workflow

### Adding a colormap checklist:
- [ ] Create or generate colormap JSON
- [ ] Customize name and positions
- [ ] Register in `src/io.rs` macro
- [ ] Run `cargo test` to verify
- [ ] Test in `basic_usage` example
- [ ] Test in forma-fractalis if applicable
- [ ] Document in CHANGELOG.md
- [ ] Commit with descriptive message

### Release workflow:
- Update version in Cargo.toml
- Update CHANGELOG.md with new colormaps
- Run full test suite: `cargo test`
- Build examples: `cargo build --examples`
- Tag release: `git tag v0.x.y`
- Publish: `cargo publish`
- Push tags: `git push --tags`
