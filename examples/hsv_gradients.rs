//! HSV color space gradient generation
//!
//! This example demonstrates creating gradients using the HSV (Hue, Saturation, Value)
//! color space, which is often more intuitive for creating vibrant, perceptually
//! uniform color transitions.

use scala_chromatica::{Color, ColorMap, ColorStop};

fn main() {
    println!("=== HSV Color Space Examples ===\n");

    // Example 1: Rainbow gradient using HSV
    println!("1. Rainbow gradient (full hue rotation):");
    let mut rainbow = ColorMap::new("HSV Rainbow");
    
    // Create stops by rotating through hue (0-360 degrees)
    for i in 0..=6 {
        let position = i as f64 / 6.0;
        let hue = i as f64 * 60.0; // 0, 60, 120, 180, 240, 300, 360
        let color = Color::from_hsv(hue, 1.0, 1.0); // Full saturation and value
        rainbow.add_stop(ColorStop::new(position, color));
    }
    
    // Sample the rainbow
    for i in 0..=10 {
        let pos = i as f64 / 10.0;
        let color = rainbow.get_color(pos);
        print_color_bar(pos, color);
    }
    println!();

    // Example 2: Saturated vs desaturated gradients
    println!("\n2. Saturation comparison (Blue to Yellow):");
    
    // High saturation version
    println!("   High saturation:");
    let mut saturated = ColorMap::new("Saturated");
    saturated.add_stop(ColorStop::new(0.0, Color::from_hsv(240.0, 1.0, 1.0))); // Pure blue
    saturated.add_stop(ColorStop::new(1.0, Color::from_hsv(60.0, 1.0, 1.0)));  // Pure yellow
    
    for i in 0..=10 {
        let pos = i as f64 / 10.0;
        let color = saturated.get_color(pos);
        print_color_bar(pos, color);
    }
    
    // Low saturation version
    println!("\n   Low saturation (pastel):");
    let mut pastel = ColorMap::new("Pastel");
    pastel.add_stop(ColorStop::new(0.0, Color::from_hsv(240.0, 0.3, 1.0))); // Pastel blue
    pastel.add_stop(ColorStop::new(1.0, Color::from_hsv(60.0, 0.3, 1.0)));  // Pastel yellow
    
    for i in 0..=10 {
        let pos = i as f64 / 10.0;
        let color = pastel.get_color(pos);
        print_color_bar(pos, color);
    }
    println!();

    // Example 3: Value (brightness) gradient
    println!("\n3. Brightness gradient (constant hue - red):");
    let mut brightness = ColorMap::new("Brightness");
    brightness.add_stop(ColorStop::new(0.0, Color::from_hsv(0.0, 1.0, 0.0)));   // Black
    brightness.add_stop(ColorStop::new(0.5, Color::from_hsv(0.0, 1.0, 0.5)));   // Dark red
    brightness.add_stop(ColorStop::new(1.0, Color::from_hsv(0.0, 1.0, 1.0)));   // Bright red
    
    for i in 0..=10 {
        let pos = i as f64 / 10.0;
        let color = brightness.get_color(pos);
        print_color_bar(pos, color);
    }
    println!();

    // Example 4: Complementary colors (opposite on color wheel)
    println!("\n4. Complementary color transitions:");
    let complementary_pairs = [
        ("Red → Cyan", 0.0, 180.0),
        ("Orange → Blue", 30.0, 210.0),
        ("Yellow → Purple", 60.0, 240.0),
    ];
    
    for (name, hue1, hue2) in complementary_pairs {
        println!("   {}:", name);
        let mut gradient = ColorMap::new(name);
        gradient.add_stop(ColorStop::new(0.0, Color::from_hsv(hue1, 1.0, 1.0)));
        gradient.add_stop(ColorStop::new(1.0, Color::from_hsv(hue2, 1.0, 1.0)));
        
        for i in 0..=10 {
            let pos = i as f64 / 10.0;
            let color = gradient.get_color(pos);
            print_color_bar(pos, color);
        }
        println!();
    }

    // Example 5: Save an HSV-based gradient
    println!("\n5. Saving custom HSV gradient...");
    let mut sunset = ColorMap::new("HSV Sunset");
    sunset.add_stop(ColorStop::new(0.0, Color::from_hsv(240.0, 0.8, 0.3))); // Deep blue
    sunset.add_stop(ColorStop::new(0.3, Color::from_hsv(280.0, 0.6, 0.5))); // Purple
    sunset.add_stop(ColorStop::new(0.6, Color::from_hsv(20.0, 1.0, 1.0)));  // Orange
    sunset.add_stop(ColorStop::new(1.0, Color::from_hsv(60.0, 1.0, 1.0)));  // Yellow
    
    match scala_chromatica::io::save_colormap(&sunset) {
        Ok(_) => println!("   ✓ Saved 'HSV Sunset' gradient to disk"),
        Err(e) => println!("   ✗ Error saving: {}", e),
    }
}

/// Print a visual representation of a color
fn print_color_bar(position: f64, color: Color) {
    println!(
        "  {:.1}  RGB({:3}, {:3}, {:3})  {}",
        position,
        color.r,
        color.g,
        color.b,
        color_block(color)
    );
}

/// Generate a simple ASCII color block representation
fn color_block(color: Color) -> String {
    // Simple grayscale approximation for terminal display
    let brightness = (color.r as u32 + color.g as u32 + color.b as u32) / 3;
    let char = if brightness > 200 {
        '█'
    } else if brightness > 150 {
        '▓'
    } else if brightness > 100 {
        '▒'
    } else if brightness > 50 {
        '░'
    } else {
        '·'
    };
    format!("{}{}{}", char, char, char)
}
