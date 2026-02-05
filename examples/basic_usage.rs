//! Basic usage example showing colormap loading and color sampling

use scala_chromatica::{Color, ColorMap, ColorStop};

fn main() {
    // Load built-in colormap
    println!("Loading Fire colormap...");
    let fire = scala_chromatica::io::load_builtin_colormap("Fire")
        .expect("Failed to load Fire colormap");

    // Sample colors at different positions
    println!("\nFire gradient samples:");
    for i in 0..=10 {
        let position = i as f64 / 10.0;
        let color = fire.get_color(position);
        println!("  {:.1}: RGB({}, {}, {})", position, color.r, color.g, color.b);
    }

    // Create custom gradient
    println!("\nCreating custom gradient...");
    let mut custom = ColorMap::new("RedBlue");
    custom.add_stop(ColorStop::new(0.0, Color::new(255, 0, 0)));
    custom.add_stop(ColorStop::new(0.5, Color::new(255, 255, 255)));
    custom.add_stop(ColorStop::new(1.0, Color::new(0, 0, 255)));

    println!("\nCustom gradient samples:");
    for i in 0..=10 {
        let position = i as f64 / 10.0;
        let color = custom.get_color(position);
        println!("  {:.1}: RGB({}, {}, {})", position, color.r, color.g, color.b);
    }

    // List all available colormaps
    println!("\nListing all available colormaps...");
    match scala_chromatica::io::list_available_colormaps() {
        Ok(colormaps) => {
            println!("Found {} colormap(s):", colormaps.len());
            for info in colormaps {
                let type_str = if info.is_builtin {
                    "built-in"
                } else {
                    "custom"
                };
                println!("  - {} ({})", info.name, type_str);
            }
        }
        Err(e) => eprintln!("Error listing colormaps: {}", e),
    }

    // Demonstrate HSV color creation
    println!("\nHSV color examples:");
    let red = Color::from_hsv(0.0, 1.0, 1.0);
    println!("  Red (H=0):   RGB({}, {}, {})", red.r, red.g, red.b);

    let green = Color::from_hsv(120.0, 1.0, 1.0);
    println!("  Green (H=120): RGB({}, {}, {})", green.r, green.g, green.b);

    let blue = Color::from_hsv(240.0, 1.0, 1.0);
    println!("  Blue (H=240): RGB({}, {}, {})", blue.r, blue.g, blue.b);
}
