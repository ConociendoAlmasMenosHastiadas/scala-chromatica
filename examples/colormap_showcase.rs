use eframe::egui;
use scala_chromatica::io;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 700.0])
            .with_title("scala-chromatica Colormap Showcase"),
        ..Default::default()
    };

    eframe::run_native(
        "Colormap Showcase",
        options,
        Box::new(|_cc| Ok(Box::new(ColormapShowcase::default()))),
    )
}

#[derive(Default)]
struct ColormapShowcase {
    selected_colormap: Option<String>,
}

impl eframe::App for ColormapShowcase {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(10.0);
            ui.vertical_centered(|ui| {
                ui.heading(egui::RichText::new("scala-chromatica").size(28.0).strong());
                ui.label(
                    egui::RichText::new("Colormap Showcase")
                        .size(16.0)
                        .color(egui::Color32::GRAY),
                );
            });
            ui.add_space(5.0);
            ui.separator();
            ui.add_space(10.0);

            ui.label(
                "Click on a colormap to view details. White circles mark color stop positions where interpolation anchors are defined.",
            );
            ui.add_space(10.0);

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.spacing_mut().item_spacing.y = 8.0;

                // Get all builtin colormap names
                let colormap_names = get_all_builtin_colormaps();

                for name in colormap_names {
                    let is_selected = self.selected_colormap.as_ref() == Some(&name);

                    // Container frame for each colormap
                    let frame = egui::Frame::none()
                        .inner_margin(egui::Margin::same(8.0))
                        .fill(if is_selected {
                            ui.visuals().faint_bg_color
                        } else {
                            egui::Color32::TRANSPARENT
                        })
                        .rounding(4.0);

                    frame.show(ui, |ui| {
                        // Colormap name header
                        let name_response = ui
                            .horizontal(|ui| {
                                if is_selected {
                                    ui.label(
                                        egui::RichText::new("▶")
                                            .color(ui.visuals().hyperlink_color),
                                    );
                                }
                                ui.label(egui::RichText::new(&name).size(15.0).strong());
                            })
                            .response;

                        // Make the name clickable
                        if name_response.interact(egui::Sense::click()).clicked() {
                            self.selected_colormap = if is_selected {
                                None // Deselect if already selected
                            } else {
                                Some(name.clone())
                            };
                        }

                        ui.add_space(4.0);

                        // Load and display the colormap
                        if let Ok(colormap) = io::load_builtin_colormap(&name) {
                            let gradient_height = if is_selected { 50.0 } else { 35.0 };
                            draw_colormap_gradient(ui, &colormap, gradient_height);

                            // Show additional info if selected
                            if is_selected {
                                ui.add_space(8.0);
                                ui.horizontal(|ui| {
                                    ui.label(
                                        egui::RichText::new(format!(
                                            "{} color stops",
                                            colormap.stops.len()
                                        ))
                                        .size(12.0),
                                    );
                                });

                                ui.add_space(4.0);

                                // Show color stops in a nice grid
                                ui.horizontal_wrapped(|ui| {
                                    for stop in &colormap.stops {
                                        let color = stop.color;
                                        let egui_color =
                                            egui::Color32::from_rgb(color.r, color.g, color.b);
                                        let hex_text = format!(
                                            "#{:02X}{:02X}{:02X}",
                                            color.r, color.g, color.b
                                        );

                                        // Create a small colored box with the hex code
                                        ui.scope(|ui| {
                                            let frame = egui::Frame::none()
                                                .fill(egui_color)
                                                .inner_margin(egui::Margin::symmetric(6.0, 4.0))
                                                .rounding(3.0);

                                            frame.show(ui, |ui| {
                                                // Choose text color based on brightness
                                                let brightness = (color.r as f32 * 0.299
                                                    + color.g as f32 * 0.587
                                                    + color.b as f32 * 0.114)
                                                    / 255.0;
                                                let text_color = if brightness > 0.5 {
                                                    egui::Color32::BLACK
                                                } else {
                                                    egui::Color32::WHITE
                                                };
                                                ui.label(
                                                    egui::RichText::new(hex_text)
                                                        .size(11.0)
                                                        .color(text_color)
                                                        .monospace(),
                                                );
                                            });
                                        });
                                    }
                                });
                            }
                        }
                    });
                }
            });
        });
    }
}

fn draw_colormap_gradient(ui: &mut egui::Ui, colormap: &scala_chromatica::ColorMap, height: f32) {
    let available_width = ui.available_width();
    let (response, painter) =
        ui.allocate_painter(egui::vec2(available_width, height), egui::Sense::hover());

    let rect = response.rect;

    // Use egui's mesh for smooth gradients without visible lines
    let num_segments = 512; // Higher resolution for smoother gradients
    let mut mesh = egui::Mesh::default();

    // Create a quad strip for the gradient
    for i in 0..num_segments {
        let t = i as f64 / (num_segments - 1) as f64;
        let x = rect.left() + (rect.width() * t as f32);

        let color = colormap.get_color(t);
        let egui_color = egui::Color32::from_rgb(color.r, color.g, color.b);

        // Add two vertices (top and bottom) for this position
        let top_pos = egui::pos2(x, rect.top());
        let bottom_pos = egui::pos2(x, rect.bottom());

        mesh.colored_vertex(top_pos, egui_color);
        mesh.colored_vertex(bottom_pos, egui_color);

        // Create triangles connecting to the previous vertices
        if i > 0 {
            let base_idx = ((i - 1) * 2) as u32;
            // First triangle
            mesh.add_triangle(base_idx, base_idx + 1, base_idx + 2);
            // Second triangle
            mesh.add_triangle(base_idx + 1, base_idx + 3, base_idx + 2);
        }
    }

    painter.add(egui::Shape::mesh(mesh));

    // Draw border with subtle styling
    painter.rect_stroke(rect, 2.0, (1.0, egui::Color32::from_gray(128)));

    // Draw stop indicators to highlight where color stops are positioned
    for stop in &colormap.stops {
        let x = rect.left() + (rect.width() * stop.position as f32);
        let center_y = rect.center().y;

        // Draw a small circle at each stop position
        let stop_color = egui::Color32::from_rgb(stop.color.r, stop.color.g, stop.color.b);
        let radius = 5.0;

        // Draw white circle with colored center
        painter.circle_filled(egui::pos2(x, center_y), radius + 2.0, egui::Color32::WHITE);
        painter.circle_filled(egui::pos2(x, center_y), radius, stop_color);
        painter.circle_stroke(
            egui::pos2(x, center_y),
            radius + 2.0,
            (2.0, egui::Color32::from_black_alpha(180)),
        );

        // Draw a subtle vertical line from top to bottom at the stop position
        painter.line_segment(
            [egui::pos2(x, rect.top()), egui::pos2(x, rect.bottom())],
            (1.0, egui::Color32::from_white_alpha(60)),
        );
    }
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
    ]
    .into_iter()
    .map(|s| s.to_string())
    .collect()
}
