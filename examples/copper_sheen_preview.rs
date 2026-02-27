//! Preview of Copper Sheen colormap variations
//! Run with: cargo run --example copper_sheen_preview

fn main() {
    println!("\n=== Copper Sheen Colormap Previews ===\n");
    
    let variants = [
        ("v1", include_str!("../src/colormaps/copper_sheen_v1.json")),
        ("v2", include_str!("../src/colormaps/copper_sheen_v2.json")),
        ("v3", include_str!("../src/colormaps/copper_sheen_v3.json")),
    ];
    
    for (name, json) in variants {
        let colormap: serde_json::Value = serde_json::from_str(json).unwrap();
        let stops = colormap["stops"].as_array().unwrap();
        
        println!("Copper Sheen {} - {} stops", name, stops.len());
        println!("{}", "─".repeat(60));
        
        // Print gradient bar using ANSI colors
        print!("  ");
        for i in 0..60 {
            let t = i as f64 / 59.0;
            let (r, g, b) = interpolate_color(stops, t);
            print!("\x1b[48;2;{};{};{}m \x1b[0m", r, g, b);
        }
        println!();
        
        // Print stop positions
        print!("  ");
        for stop in stops {
            let pos = stop["position"].as_f64().unwrap();
            let col = &stop["color"];
            let r = col["r"].as_u64().unwrap() as u8;
            let g = col["g"].as_u64().unwrap() as u8;
            let b = col["b"].as_u64().unwrap() as u8;
            print!("\x1b[38;2;{};{};{}m│\x1b[0m", r, g, b);
            print!("{:.2} ", pos);
        }
        println!("\n");
        
        // Print description
        println!("  Stops:");
        for (i, stop) in stops.iter().enumerate() {
            let pos = stop["position"].as_f64().unwrap();
            let col = &stop["color"];
            let r = col["r"].as_u64().unwrap() as u8;
            let g = col["g"].as_u64().unwrap() as u8;
            let b = col["b"].as_u64().unwrap() as u8;
            let desc = describe_color(r, g, b);
            print!("    \x1b[48;2;{};{};{}m  \x1b[0m", r, g, b);
            println!(" {}: pos={:.2} RGB({},{},{}) - {}", i, pos, r, g, b, desc);
        }
        println!("\n");
    }
    
    println!("Which variation do you prefer? Comment in v0.1.3.md!");
}

fn interpolate_color(stops: &[serde_json::Value], t: f64) -> (u8, u8, u8) {
    let t = t.clamp(0.0, 1.0);
    
    // Find surrounding stops
    let mut prev_idx = 0;
    for (i, stop) in stops.iter().enumerate() {
        if stop["position"].as_f64().unwrap() <= t {
            prev_idx = i;
        }
    }
    
    let next_idx = (prev_idx + 1).min(stops.len() - 1);
    
    let prev = &stops[prev_idx];
    let next = &stops[next_idx];
    
    let prev_pos = prev["position"].as_f64().unwrap();
    let next_pos = next["position"].as_f64().unwrap();
    
    let local_t = if (next_pos - prev_pos).abs() < 0.0001 {
        0.0
    } else {
        (t - prev_pos) / (next_pos - prev_pos)
    };
    
    let lerp = |a: u8, b: u8| -> u8 {
        ((a as f64) * (1.0 - local_t) + (b as f64) * local_t) as u8
    };
    
    let pr = prev["color"]["r"].as_u64().unwrap() as u8;
    let pg = prev["color"]["g"].as_u64().unwrap() as u8;
    let pb = prev["color"]["b"].as_u64().unwrap() as u8;
    let nr = next["color"]["r"].as_u64().unwrap() as u8;
    let ng = next["color"]["g"].as_u64().unwrap() as u8;
    let nb = next["color"]["b"].as_u64().unwrap() as u8;
    
    (lerp(pr, nr), lerp(pg, ng), lerp(pb, nb))
}

fn describe_color(r: u8, g: u8, b: u8) -> &'static str {
    let brightness = (r as u16 + g as u16 + b as u16) / 3;
    
    if brightness < 20 {
        return "near-black";
    }
    if brightness > 245 {
        return "near-white";
    }
    
    // Check for teal/turquoise
    if g > r && b > r && g > 100 && b > 100 {
        if g > b { "aquamarine/turquoise" } else { "teal/cyan" }
    }
    // Check for copper/brown
    else if r > g && r > b && g > b {
        if brightness > 180 { "light copper/peach" }
        else if brightness > 120 { "copper/bronze" }
        else { "dark copper/sienna" }
    }
    // Check for white-ish
    else if brightness > 200 && (r as i16 - g as i16).abs() < 30 && (g as i16 - b as i16).abs() < 30 {
        "off-white/cream"
    }
    else {
        "transition"
    }
}
