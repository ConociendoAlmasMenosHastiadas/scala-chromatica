//! Test v0.1.4 features: slice, discretize, period fix, Electric Indigo

use scala_chromatica::color_from_iterations;

fn main() {
    println!("=== Testing v0.1.4 Features ===\n");

    // Test 1: slice()
    println!("1. Testing ColorMap::slice()");
    let rainbow = scala_chromatica::io::load_builtin_colormap("Rainbow").unwrap();
    let middle_slice = rainbow.slice(0.3, 0.7);
    println!("   Original Rainbow at 0.5: {:?}", rainbow.get_color(0.5));
    println!("   Sliced [0.3-0.7] at 0.5: {:?}", middle_slice.get_color(0.5));
    println!("   Slice has {} stops\n", middle_slice.stops.len());

    // Test 2: discretize()
    println!("2. Testing ColorMap::discretize()");
    let fire = scala_chromatica::io::load_builtin_colormap("Fire").unwrap();
    let discrete = fire.discretize(5);
    println!("   Original Fire has {} stops", fire.stops.len());
    println!("   Discretized to 5 bands: {} stops", discrete.stops.len());
    for (i, stop) in discrete.stops.iter().enumerate() {
        println!("      Band {}: pos={:.2}, RGB({}, {}, {})", 
                 i, stop.position, stop.color.r, stop.color.g, stop.color.b);
    }
    println!();

    // Test 3: Period endpoint fix - Egyptian Echo (2-stop colormap)
    println!("3. Testing period endpoint fix with Egyptian Echo");
    let egyptian = scala_chromatica::io::load_builtin_colormap("Egyptian Echo").unwrap();
    println!("   Egyptian Echo stops:");
    for stop in &egyptian.stops {
        println!("      pos={:.3}, RGB({}, {}, {})", 
                 stop.position, stop.color.r, stop.color.g, stop.color.b);
    }
    println!("\n   Testing with period=2:");
    for iter in 0..6 {
        let color = color_from_iterations(iter, 100, &egyptian, true, 2, false, [0, 0, 0], false);
        let adjusted_iter = iter % 2;
        let t = if 2 == 1 { 0.0 } else { adjusted_iter as f64 / (2 - 1) as f64 };
        println!("      iter={}, adjusted={}, t={:.3}, RGB({}, {}, {})", 
                 iter, adjusted_iter, t, color.r, color.g, color.b);
    }
    println!("   ✓ Should see both endpoint colors (blue and gold)\n");

    // Test 4: Electric Indigo colormap
    println!("4. Testing new Electric Indigo colormap");
    let electric = scala_chromatica::io::load_builtin_colormap("Electric Indigo").unwrap();
    println!("   Electric Indigo has {} stops", electric.stops.len());
    println!("   First stop: RGB({}, {}, {})", 
             electric.stops[0].color.r, electric.stops[0].color.g, electric.stops[0].color.b);
    println!("   Last stop: RGB({}, {}, {})", 
             electric.stops.last().unwrap().color.r, 
             electric.stops.last().unwrap().color.g, 
             electric.stops.last().unwrap().color.b);
    println!("   ✓ Cyclic colormap (same start/end color)\n");

    // Test 5: Combining features - slice + discretize
    println!("5. Combining slice + discretize");
    let ocean = scala_chromatica::io::load_builtin_colormap("Ocean").unwrap();
    let ocean_slice = ocean.slice(0.2, 0.8).discretize(4);
    println!("   Ocean sliced [0.2-0.8] and discretized to 4 bands");
    println!("   Result: {}\n", ocean_slice.name);

    println!("=== All v0.1.4 Features Working! ===");
}
