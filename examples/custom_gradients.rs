//! Advanced gradient manipulation techniques
//!
//! This example demonstrates advanced features of scala-chromatica including:
//! - Multi-stop gradients with precise positioning
//! - Color interpolation at arbitrary positions
//! - Saving and loading custom gradients
//! - Gradient inversion and manipulation

use scala_chromatica::{Color, ColorMap, ColorStop};

fn main() {
    println!("=== Advanced Gradient Techniques ===\n");

    // Example 1: Multi-stop gradient with uneven spacing
    println!("1. Non-uniform gradient (emphasis on center):");
    let mut emphasis = ColorMap::new("Center Emphasis");
    emphasis.add_stop(ColorStop::new(0.0, Color::new(0, 0, 128)));     // Dark blue
    emphasis.add_stop(ColorStop::new(0.45, Color::new(0, 100, 255)));  // Blue
    emphasis.add_stop(ColorStop::new(0.5, Color::new(255, 255, 255))); // White (center)
    emphasis.add_stop(ColorStop::new(0.55, Color::new(255, 100, 0)));  // Orange
    emphasis.add_stop(ColorStop::new(1.0, Color::new(128, 0, 0)));     // Dark red
    
    sample_gradient(&emphasis);

    // Example 2: Sharp transitions vs smooth transitions
    println!("\n2. Sharp vs smooth transitions:");
    
    println!("   Sharp (adjacent stops):");
    let mut sharp = ColorMap::new("Sharp");
    sharp.add_stop(ColorStop::new(0.0, Color::new(255, 0, 0)));
    sharp.add_stop(ColorStop::new(0.333, Color::new(255, 0, 0)));
    sharp.add_stop(ColorStop::new(0.334, Color::new(0, 255, 0)));
    sharp.add_stop(ColorStop::new(0.666, Color::new(0, 255, 0)));
    sharp.add_stop(ColorStop::new(0.667, Color::new(0, 0, 255)));
    sharp.add_stop(ColorStop::new(1.0, Color::new(0, 0, 255)));
    sample_gradient(&sharp);
    
    println!("\n   Smooth (spaced stops):");
    let mut smooth = ColorMap::new("Smooth");
    smooth.add_stop(ColorStop::new(0.0, Color::new(255, 0, 0)));
    smooth.add_stop(ColorStop::new(0.5, Color::new(0, 255, 0)));
    smooth.add_stop(ColorStop::new(1.0, Color::new(0, 0, 255)));
    sample_gradient(&smooth);

    // Example 3: Metallic gradient with named stops
    println!("\n3. Metallic gradient with named colors:");
    let mut metallic = ColorMap::new("Bronze Metal");
    metallic.add_stop(ColorStop {
        position: 0.0,
        color: Color::new(52, 28, 11),
        name: Some("Deep Bronze".to_string()),
    });
    metallic.add_stop(ColorStop {
        position: 0.3,
        color: Color::new(140, 82, 33),
        name: Some("Bronze Base".to_string()),
    });
    metallic.add_stop(ColorStop {
        position: 0.5,
        color: Color::new(205, 127, 50),
        name: Some("Bronze Highlight".to_string()),
    });
    metallic.add_stop(ColorStop {
        position: 0.7,
        color: Color::new(140, 82, 33),
        name: Some("Bronze Shadow".to_string()),
    });
    metallic.add_stop(ColorStop {
        position: 1.0,
        color: Color::new(52, 28, 11),
        name: Some("Deep Bronze".to_string()),
    });
    
    sample_gradient(&metallic);
    
    // Print stop information
    println!("   Color stops:");
    for (i, stop) in metallic.stops.iter().enumerate() {
        let name = stop.name.as_deref().unwrap_or("Unnamed");
        println!(
            "     Stop {}: {:.2} - {} - RGB({}, {}, {})",
            i, stop.position, name, stop.color.r, stop.color.g, stop.color.b
        );
    }

    // Example 4: Color interpolation at specific positions
    println!("\n4. Precise color sampling:");
    let mut gradient = ColorMap::new("Test");
    gradient.add_stop(ColorStop::new(0.0, Color::new(0, 0, 0)));
    gradient.add_stop(ColorStop::new(1.0, Color::new(255, 255, 255)));
    
    let positions = [0.0, 0.25, 0.5, 0.75, 1.0];
    for &pos in &positions {
        let color = gradient.get_color(pos);
        println!(
            "   Position {:.2}: RGB({:3}, {:3}, {:3})",
            pos, color.r, color.g, color.b
        );
    }

    // Example 5: Gradient manipulation - creating an inverted version
    println!("\n5. Gradient inversion:");
    let mut original = ColorMap::new("Original");
    original.add_stop(ColorStop::new(0.0, Color::new(255, 0, 0)));
    original.add_stop(ColorStop::new(0.5, Color::new(0, 255, 0)));
    original.add_stop(ColorStop::new(1.0, Color::new(0, 0, 255)));
    
    println!("   Original:");
    sample_gradient(&original);
    
    // Create inverted version by reversing stop positions
    let mut inverted = ColorMap::new("Inverted");
    for stop in original.stops.iter().rev() {
        inverted.add_stop(ColorStop::new(1.0 - stop.position, stop.color));
    }
    
    println!("\n   Inverted:");
    sample_gradient(&inverted);

    // Example 6: Save and load demonstration
    println!("\n6. Save/Load custom gradients:");
    
    // Create a gradient
    let mut custom = ColorMap::new("MyCustomGradient");
    custom.add_stop(ColorStop::new(0.0, Color::from_hsv(180.0, 1.0, 0.5)));
    custom.add_stop(ColorStop::new(0.5, Color::from_hsv(220.0, 0.8, 0.7)));
    custom.add_stop(ColorStop::new(1.0, Color::from_hsv(260.0, 1.0, 1.0)));
    
    // Save it
    match scala_chromatica::io::save_colormap(&custom) {
        Ok(_) => {
            println!("   ✓ Saved 'MyCustomGradient'");
            
            // Try to load it back
            match scala_chromatica::io::load_colormap("MyCustomGradient") {
                Ok(loaded) => {
                    println!("   ✓ Loaded back successfully");
                    println!("     Name: {}", loaded.name);
                    println!("     Stops: {}", loaded.stops.len());
                }
                Err(e) => println!("   ✗ Failed to load: {}", e),
            }
        }
        Err(e) => println!("   ✗ Failed to save: {}", e),
    }

    // Example 7: List all available gradients
    println!("\n7. Available gradients:");
    match scala_chromatica::io::list_available_colormaps() {
        Ok(colormaps) => {
            let builtin: Vec<_> = colormaps
                .iter()
                .filter(|c| c.is_builtin)
                .map(|c| c.name.as_str())
                .collect();
            let custom: Vec<_> = colormaps
                .iter()
                .filter(|c| !c.is_builtin)
                .map(|c| c.name.as_str())
                .collect();
            
            println!("   Built-in ({}):", builtin.len());
            for name in builtin {
                println!("     - {}", name);
            }
            
            if !custom.is_empty() {
                println!("   Custom ({}):", custom.len());
                for name in custom {
                    println!("     - {}", name);
                }
            }
        }
        Err(e) => println!("   Error: {}", e),
    }
}

/// Sample a gradient and print the results
fn sample_gradient(gradient: &ColorMap) {
    for i in 0..=10 {
        let pos = i as f64 / 10.0;
        let color = gradient.get_color(pos);
        println!(
            "   {:.1}  RGB({:3}, {:3}, {:3})",
            pos, color.r, color.g, color.b
        );
    }
}
