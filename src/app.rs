use crate::alphabet::default_alphabet;
use crate::canvas::render_vector_canvas;
use eframe::egui;
use egui::{pos2, vec2, Color32, Rect, Rounding, Sense};
use std::collections::HashMap;

pub struct FontEditorApp {
    pub col_widths: [f32; 3],
    pub row_heights: [f32; 5],
    pub gap: f32,
    pub corner_radius: f32,
    pub alphabet: HashMap<char, Vec<usize>>,
}

impl Default for FontEditorApp {
    fn default() -> Self {
        Self {
            col_widths: [60.0, 60.0, 60.0],
            row_heights: [45.0, 45.0, 45.0, 45.0, 45.0],
            gap: 4.0,
            corner_radius: 6.0,
            alphabet: default_alphabet(),
        }
    }
}

impl eframe::App for FontEditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Left Side Panel: Interactive Vector Canvas
        egui::SidePanel::left("control_panel")
            .resizable(false)
            .default_width(420.0)
            .show(ctx, |ui| {
                render_vector_canvas(ui, self);

                ui.add_space(16.0);
                ui.separator();
                ui.label("Global Properties:");
                ui.add(egui::Slider::new(&mut self.gap, 0.0..=15.0).text("Box Gap"));
                ui.add(
                    egui::Slider::new(&mut self.corner_radius, 0.0..=20.0).text("Corner Radius"),
                );
            });

        // Right Panel: Live Alphabet Grid
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Live Font Output (A-Z)");
            ui.add_space(10.0);

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    let mut sorted_keys: Vec<_> = self.alphabet.keys().cloned().collect();
                    sorted_keys.sort();

                    for ch in sorted_keys {
                        let active_boxes = &self.alphabet[&ch];

                        ui.vertical(|ui| {
                            let card_size = vec2(70.0, 95.0);
                            let (response, painter) =
                                ui.allocate_painter(card_size, Sense::hover());

                            painter.rect_filled(
                                response.rect,
                                Rounding::same(6.0_f32),
                                Color32::from_rgb(24, 24, 28),
                            );

                            let total_w: f32 =
                                self.col_widths.iter().sum::<f32>() + (self.gap * 2.0);
                            let total_h: f32 =
                                self.row_heights.iter().sum::<f32>() + (self.gap * 4.0);
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
                                        let rect =
                                            Rect::from_min_size(pos2(curr_x, curr_y), vec2(w, h));
                                        painter.rect_filled(
                                            rect,
                                            Rounding::same(self.corner_radius * scale),
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
