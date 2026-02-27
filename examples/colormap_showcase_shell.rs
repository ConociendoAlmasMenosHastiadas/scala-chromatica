//! Terminal-based showcase of all built-in colormaps
//! Displays each colormap with ANSI colors, gradient bar, and detailed stop information
//! Run with: cargo run --example colormap_showcase_shell

use scala_chromatica::io::load_builtin_colormap;

fn main() {
    println!("\n╔═══════════════════════════════════════════════════════════╗");
    println!("║          scala-chromatica Built-in Colormaps             ║");
    println!("╚═══════════════════════════════════════════════════════════╝\n");

    let colormap_names = get_all_builtin_colormaps();
    
    for (idx, name) in colormap_names.iter().enumerate() {
        println!("─────────────────────────────────────────────────────────────");
        println!("Colormap #{}: {}", idx + 1, name);
        println!("─────────────────────────────────────────────────────────────");
        
        match load_builtin_colormap(name) {
            Ok(colormap) => {
                // Print gradient bar using ANSI colors
                print!("Gradient: ");
                for i in 0..80 {
                    let t = i as f64 / 79.0;
                    let color = colormap.get_color(t);
                    print!("\x1b[48;2;{};{};{}m \x1b[0m", color.r, color.g, color.b);
                }
                println!();
                
                // Print stops count
                println!("Stops:    {} color stops", colormap.stops.len());
                println!();
                
                // Print detailed stop information
                for (i, stop) in colormap.stops.iter().enumerate() {
                    let color = &stop.color;
                    let desc = describe_color(color.r, color.g, color.b);
                    
                    // Color swatch
                    print!("  ");
                    print!("\x1b[48;2;{};{};{}m    \x1b[0m", color.r, color.g, color.b);
                    
                    // Stop details
                    print!(" [{:2}] pos={:.3}", i, stop.position);
                    print!("  RGB({:3},{:3},{:3})", color.r, color.g, color.b);
                    print!("  {}", color.to_hex());
                    println!("  {}", desc);
                }
                
                println!();
            }
            Err(e) => {
                println!("  ERROR: Failed to load colormap: {}", e);
                println!();
            }
        }
    }
    
    println!("─────────────────────────────────────────────────────────────");
    println!("Total: {} built-in colormaps", colormap_names.len());
    println!();
}

fn get_all_builtin_colormaps() -> Vec<String> {
    // List of all builtin colormaps - must match io.rs
    vec![
        "Default",
        "Fire",
        "Ocean",
        "Grayscale",
        "Rainbow",
        "Academic",
        "Twilight Garden",
        "Coral Sunset",
        "Olive Symmetry",
        "Orchid Garden",
        "Frozen Amaranth",
        "Electric Neon",
        "Cosmic Dawn",
        "Vintage Lavender",
        "Spring Meadow",
        "Egyptian Echo",
        "Copper Sheen",
    ]
    .into_iter()
    .map(|s| s.to_string())
    .collect()
}

fn describe_color(r: u8, g: u8, b: u8) -> String {
    let brightness = (r as u16 + g as u16 + b as u16) / 3;
    let max_component = r.max(g).max(b);
    let min_component = r.min(g).min(b);
    let chroma = max_component - min_component;
    
    // Near grayscale
    if chroma < 20 {
        if brightness < 20 {
            return "near-black".to_string();
        } else if brightness > 235 {
            return "near-white".to_string();
        } else {
            return format!("gray (brightness: {})", brightness);
        }
    }
    
    // Determine dominant hue
    let dominant = if r >= g && r >= b {
        "red"
    } else if g >= r && g >= b {
        "green"
    } else {
        "blue"
    };
    
    // Determine saturation level
    let saturation = if max_component > 0 {
        (chroma as f32 / max_component as f32 * 100.0) as u32
    } else {
        0
    };
    
    // Build description
    let mut desc = String::new();
    
    // Brightness modifier
    if brightness < 80 {
        desc.push_str("dark ");
    } else if brightness > 180 {
        desc.push_str("bright ");
    }
    
    // Saturation modifier
    if saturation < 30 {
        desc.push_str("pale ");
    } else if saturation > 80 {
        desc.push_str("vivid ");
    }
    
    // Specific color names based on RGB patterns
    if r > 200 && g < 100 && b < 100 {
        desc.push_str("red");
    } else if r > 200 && g > 100 && b < 80 {
        desc.push_str("orange/yellow");
    } else if g > 200 && r < 100 && b < 100 {
        desc.push_str("green");
    } else if b > 200 && r < 100 && g < 150 {
        desc.push_str("blue");
    } else if b > 150 && g > 150 && r < 100 {
        desc.push_str("cyan/teal");
    } else if r > 150 && b > 150 && g < 100 {
        desc.push_str("magenta/purple");
    } else if r > 150 && g > 150 && b < 100 {
        desc.push_str("yellow");
    } else if r > 150 && g > 100 && b > 150 {
        desc.push_str("lavender/pink");
    } else if r > 100 && g > 50 && b < 50 {
        desc.push_str("brown/copper");
    } else {
        desc.push_str(dominant);
        desc.push_str(" tint");
    }
    
    desc
}
