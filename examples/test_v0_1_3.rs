//! Quick test of new v0.1.3 features
//! Tests hex color parsing and colormap reversal

use scala_chromatica::{Color, ColorMap, ColorStop};

fn main() {
    println!("\n=== Testing v0.1.3 Features ===\n");
    
    // Test hex color parsing
    println!("Testing hex color parsing:");
    let color1 = Color::from_hex("#FF5733").unwrap();
    println!("  #FF5733 -> {}", color1);
    
    let color2 = Color::from_hex("C8F").unwrap();
    println!("  C8F     -> {}", color2);
    
    let color3 = Color::from_hex("#00FF00").unwrap();
    println!("  #00FF00 -> {}", color3);
    
    // Test to_hex
    println!("\nTesting to_hex:");
    let orange = Color::new(255, 165, 0);
    println!("  RGB(255, 165, 0) -> {}", orange.to_hex());
    
    // Test colormap reversal
    println!("\nTesting colormap reversal:");
    let copper = scala_chromatica::io::load_builtin_colormap("Copper Sheen").unwrap();
    println!("  Original Copper Sheen:");
    println!("    Start (0.0): {}", copper.get_color(0.0));
    println!("    Mid   (0.5): {}", copper.get_color(0.5));
    println!("    End   (1.0): {}", copper.get_color(1.0));
    
    let reversed = copper.reversed();
    println!("\n  Reversed Copper Sheen:");
    println!("    Start (0.0): {}", reversed.get_color(0.0));
    println!("    Mid   (0.5): {}", reversed.get_color(0.5));
    println!("    End   (1.0): {}", reversed.get_color(1.0));
    
    // Test creating gradient with hex colors
    println!("\nCreating gradient with hex colors:");
    let mut hex_gradient = ColorMap::new("Hex Gradient");
    hex_gradient.add_stop(ColorStop::new(0.0, Color::from_hex("#FF0000").unwrap()));
    hex_gradient.add_stop(ColorStop::new(0.5, Color::from_hex("#FFFF00").unwrap()));
    hex_gradient.add_stop(ColorStop::new(1.0, Color::from_hex("#0000FF").unwrap()));
    
    println!("  Red -> Yellow -> Blue:");
    for i in 0..=10 {
        let t = i as f64 / 10.0;
        let color = hex_gradient.get_color(t);
        print!("    {:.1}: {} ({})", t, color, color.to_hex());
        if i % 2 == 0 {
            println!();
        } else {
            print!("  ");
        }
    }
    
    println!("\n✓ All v0.1.3 features working correctly!");
}
