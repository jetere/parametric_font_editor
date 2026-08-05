use eframe::egui;
use egui::{pos2, vec2, Color32, CornerRadius, Pos2, Rect, Sense, Stroke};
use std::collections::HashMap;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1100.0, 700.0])
            .with_title("15-Box Parametric Font Studio (Rust Native)"),
        ..Default::default()
    };

    eframe::run_native(
        "Parametric Font Studio",
        native_options,
        Box::new(|_cc| Ok(Box::new(FontEditorApp::default()))),
    )
}

struct FontEditorApp {
    col_widths: [f32; 3],
    row_heights: [f32; 5],
    gap: f32,
    corner_radius: f32,
    alphabet: HashMap<char, Vec<usize>>,
}

impl Default for FontEditorApp {
    fn default() -> Self {
        let mut alphabet = HashMap::new();
        // 15-box indices (1 to 15) mapped to letters
        alphabet.insert('A', vec![2, 4, 6, 7, 8, 9, 10, 12]);
        alphabet.insert('B', vec![1, 2, 4, 6, 7, 8, 10, 11]);
        alphabet.insert('C', vec![1, 2, 3, 4, 7, 10, 11, 12]);
        alphabet.insert('E', vec![1, 2, 3, 4, 7, 8, 10, 11, 12]);
        alphabet.insert('F', vec![1, 2, 3, 4, 7, 8, 10]);
        alphabet.insert('H', vec![1, 3, 4, 6, 7, 8, 9, 10, 12, 13, 15]);
        alphabet.insert('I', vec![1, 2, 3, 5, 8, 11, 13, 14, 15]);
        alphabet.insert('L', vec![1, 4, 7, 10, 11, 12]);
        alphabet.insert('O', vec![1, 2, 3, 4, 6, 7, 9, 10, 11, 12]);
        alphabet.insert('P', vec![1, 2, 3, 4, 6, 7, 8, 9, 10]);
        alphabet.insert('T', vec![1, 2, 3, 5, 8, 11, 14]);
        alphabet.insert('U', vec![1, 3, 4, 6, 7, 9, 10, 11, 12]);

        Self {
            col_widths: [60.0, 60.0, 60.0],
            row_heights: [45.0, 45.0, 45.0, 45.0, 45.0],
            gap: 4.0,
            corner_radius: 6.0,
            alphabet,
        }
    }
}

impl eframe::App for FontEditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Left Side: Interactive Canvas & Sliders
        egui::SidePanel::left("control_panel")
            .resizable(false)
            .default_width(400.0)
            .show(ctx, |ui| {
                ui.heading("15-Box Grid Canvas");
                ui.label("Drag the yellow handles or adjust sliders below:");
                ui.add_space(8.0);

                // --- Canvas Area for Drag Handles ---
                let canvas_size = vec2(360.0, 320.0);
                let (response, painter) = ui.allocate_painter(canvas_size, Sense::hover());
                let origin = response.rect.min + vec2(40.0, 40.0);

                // Background
                painter.rect_filled(response.rect, 8.0, Color32::from_rgb(18, 18, 20));

                // Draw Grid Boxes
                let mut box_idx = 1;
                let mut curr_y = origin.y;
                for r in 0..5 {
                    let mut curr_x = origin.x;
                    for c in 0..3 {
                        let rect = Rect::from_min_size(
                            pos2(curr_x, curr_y),
                            vec2(self.col_widths[c], self.row_heights[r]),
                        );
                        painter.rect_stroke(
                            rect,
                            CornerRadius::from(self.corner_radius),
                            Stroke::new(1.0, Color32::from_rgb(60, 60, 70)),
                            egui::StrokeKind::Outside,
                        );
                        curr_x += self.col_widths[c] + self.gap;
                        box_idx += 1;
                    }
                    curr_y += self.row_heights[r] + self.gap;
                }

                // --- Column Drag Handles (Horizontal) ---
                let mut acc_x = origin.x + self.col_widths[0];
                for c in 0..2 {
                    let handle_pos = pos2(acc_x + self.gap / 2.0, origin.y - 15.0);
                    let handle_id = ui.make_persistent_id(format!("col_drag_{}", c));
                    let handle_response = ui.interact(
                        Rect::from_center_size(handle_pos, vec2(16.0, 16.0)),
                        handle_id,
                        Sense::drag(),
                    );

                    if handle_response.dragged() {
                        self.col_widths[c] = (self.col_widths[c] + handle_response.drag_delta().x).max(15.0);
                    }

                    let color = if handle_response.hovered() || handle_response.dragged() {
                        Color32::YELLOW
                    } else {
                        Color32::from_rgb(200, 200, 100)
                    };
                    painter.circle_filled(handle_pos, 6.0, color);

                    acc_x += self.col_widths[c + 1] + self.gap;
                }

                // --- Row Drag Handles (Vertical) ---
                let mut acc_y = origin.y + self.row_heights[0];
                for r in 0..4 {
                    let handle_pos = pos2(origin.x - 15.0, acc_y + self.gap / 2.0);
                    let handle_id = ui.make_persistent_id(format!("row_drag_{}", r));
                    let handle_response = ui.interact(
                        Rect::from_center_size(handle_pos, vec2(16.0, 16.0)),
                        handle_id,
                        Sense::drag(),
                    );

                    if handle_response.dragged() {
                        self.row_heights[r] = (self.row_heights[r] + handle_response.drag_delta().y).max(15.0);
                    }

                    let color = if handle_response.hovered() || handle_response.dragged() {
                        Color32::YELLOW
                    } else {
                        Color32::from_rgb(200, 200, 100)
                    };
                    painter.circle_filled(handle_pos, 6.0, color);

                    acc_y += self.row_heights[r + 1] + self.gap;
                }

                ui.add_space(12.0);
                ui.separator();
                ui.label("Fine Tuning Controls:");
                ui.add(egui::Slider::new(&mut self.gap, 0.0..=15.0).text("Box Gap"));
                ui.add(egui::Slider::new(&mut self.corner_radius, 0.0..=20.0).text("Corner Radius"));
            });

        // Right Side: Live Font Preview
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Live Font Output");
            ui.add_space(10.0);

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    let mut sorted_keys: Vec<_> = self.alphabet.keys().cloned().collect();
                    sorted_keys.sort();

                    for ch in sorted_keys {
                        let active_boxes = &self.alphabet[&ch];

                        ui.vertical(|ui| {
                            let card_size = vec2(70.0, 95.0);
                            let (response, painter) = ui.allocate_painter(card_size, Sense::hover());

                            // Card Background
                            painter.rect_filled(response.rect, 6.0, Color32::from_rgb(24, 24, 28));

                            // Scale down global 15-box dimensions to fit inside preview card
                            let total_w: f32 = self.col_widths.iter().sum::<f32>() + (self.gap * 2.0);
                            let total_h: f32 = self.row_heights.iter().sum::<f32>() + (self.gap * 4.0);
                            let scale = (50.0 / total_w).min(60.0 / total_h);

                            let card_origin = response.rect.min + vec2(10.0, 10.0);

                            let mut box_idx = 1;
                            let mut curr_y = card_origin.y;

                            for r in 0..5 {
                                let mut curr_x = card_origin.x;
                                for c in 0..3 {
                                    let w = self.col_widths[c] * scale;
                                    let h = self.row_heights[r] * scale;

                                    if active_boxes.contains(&box_idx) {
                                        let rect = Rect::from_min_size(pos2(curr_x, curr_y), vec2(w, h));
                                        painter.rect_filled(
                                            rect,
                                            CornerRadius::from(self.corner_radius * scale),
                                            Color32::from_rgb(129, 140, 248),
                                        );
                                    }

                                    curr_x += w + (self.gap * scale);
                                    box_idx += 1;
                                }
                                curr_y += (self.row_heights[r] * scale) + (self.gap * scale);
                            }

                            ui.label(ch.to_string());
                        });
                        ui.add_space(8.0);
                    }
                });
            });
        });
    }
}
