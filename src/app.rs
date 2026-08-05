use crate::alphabet::default_alphabet;
use crate::canvas::render_vector_canvas;
use crate::export::ExportManager;
use eframe::egui;
use egui::{pos2, vec2, Color32, Rect, Rounding, Sense, Stroke};
use std::collections::HashMap;

pub struct FontEditorApp {
    pub col_widths: [f32; 3],
    pub row_heights: [f32; 5],
    pub gap: f32,
    pub corner_radius: f32,
    pub aspect_ratio_locked: bool,
    pub status_msg: String,
    pub alphabet: HashMap<char, Vec<usize>>,
}

impl Default for FontEditorApp {
    fn default() -> Self {
        Self {
            col_widths: [60.0, 60.0, 60.0],
            row_heights: [45.0, 45.0, 45.0, 45.0, 45.0],
            gap: 4.0,
            corner_radius: 6.0,
            aspect_ratio_locked: false,
            status_msg: String::new(),
            alphabet: default_alphabet(),
        }
    }
}

impl eframe::App for FontEditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let orange = Color32::from_rgb(255, 102, 0);

        // Overall Application Light Theme Setup
        let mut visuals = egui::Visuals::light();
        visuals.panel_fill = Color32::from_rgb(250, 250, 250); // Clean white/light background
        visuals.override_text_color = Some(Color32::from_rgb(15, 15, 18)); // Crisp black text
        visuals.widgets.noninteractive.bg_stroke =
            Stroke::new(1.5_f32, Color32::from_rgb(200, 200, 210));
        ctx.set_visuals(visuals);

        // Left Panel: Canvas Controls & Export Options
        egui::SidePanel::left("control_panel")
            .resizable(false)
            .default_width(420.0)
            .show(ctx, |ui| {
                // Vector Canvas remains completely unchanged (Dark Artboard)
                render_vector_canvas(ui, self);

                ui.add_space(12.0);
                ui.separator();
                ui.heading("Size & Aspect Ratio Panel");
                ui.add_space(4.0);

                let mut total_w: f32 = self.col_widths.iter().sum::<f32>() + (self.gap * 2.0);
                let mut total_h: f32 = self.row_heights.iter().sum::<f32>() + (self.gap * 4.0);
                let old_w = total_w;
                let old_h = total_h;

                ui.horizontal(|ui| {
                    ui.label("Width:");
                    if ui
                        .add(
                            egui::DragValue::new(&mut total_w)
                                .clamp_range(50.0..=350.0)
                                .suffix("px"),
                        )
                        .changed()
                    {
                        let scale_x = total_w / old_w;
                        for w in self.col_widths.iter_mut() {
                            *w = (*w * scale_x).clamp(10.0, 120.0);
                        }
                        if self.aspect_ratio_locked {
                            let scale_y = scale_x;
                            for h in self.row_heights.iter_mut() {
                                *h = (*h * scale_y).clamp(10.0, 100.0);
                            }
                        }
                    }

                    ui.add_space(10.0);

                    ui.label("Height:");
                    if ui
                        .add(
                            egui::DragValue::new(&mut total_h)
                                .clamp_range(50.0..=300.0)
                                .suffix("px"),
                        )
                        .changed()
                    {
                        let scale_y = total_h / old_h;
                        for h in self.row_heights.iter_mut() {
                            *h = (*h * scale_y).clamp(10.0, 100.0);
                        }
                        if self.aspect_ratio_locked {
                            let scale_x = scale_y;
                            for w in self.col_widths.iter_mut() {
                                *w = (*w * scale_x).clamp(10.0, 120.0);
                            }
                        }
                    }
                });

                ui.horizontal(|ui| {
                    ui.checkbox(&mut self.aspect_ratio_locked, "Lock Aspect Ratio");

                    if ui.button("Square 1:1").clicked() {
                        let target = total_w.max(total_h);
                        self.col_widths = [target / 3.5, target / 3.5, target / 3.5];
                        self.row_heights = [target / 5.5; 5];
                    }
                    if ui.button("Tall 4:5").clicked() {
                        self.col_widths = [45.0, 45.0, 45.0];
                        self.row_heights = [50.0; 5];
                    }
                });

                ui.add_space(8.0);
                ui.separator();
                ui.heading("Export Tools");
                ui.add_space(4.0);

                ui.horizontal(|ui| {
                    if ui.button("🖼 Export PNG Previews").clicked() {
                        if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                            match ExportManager::export_pngs(self, &folder) {
                                Ok(count) => {
                                    self.status_msg =
                                        format!("Exported {} PNGs successfully!", count);
                                }
                                Err(err) => self.status_msg = format!("Export failed: {}", err),
                            }
                        }
                    }

                    if ui.button("🔤 Export SVG Font File").clicked() {
                        if let Some(file) = rfd::FileDialog::new()
                            .add_filter("SVG Font", &["svg"])
                            .set_file_name("ParametricFont.svg")
                            .save_file()
                        {
                            match ExportManager::export_svg_font(self, &file) {
                                Ok(_) => self.status_msg = "Font file exported!".to_string(),
                                Err(err) => {
                                    self.status_msg = format!("Font export failed: {}", err)
                                }
                            }
                        }
                    }
                });

                if !self.status_msg.is_empty() {
                    ui.add_space(4.0);
                    ui.label(egui::RichText::new(&self.status_msg).color(orange).strong());
                }

                ui.add_space(8.0);
                ui.separator();
                ui.label("Global Styling:");
                ui.add(egui::Slider::new(&mut self.gap, 0.0..=20.0).text("Box Gap"));
                ui.add(
                    egui::Slider::new(&mut self.corner_radius, 0.0..=25.0).text("Corner Radius"),
                );
            });

        // Right Panel: Live Font Output Grid
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Live Font Output (A-Z)");
            ui.add_space(10.0);

            let avail_w = ui.available_width();
            let card_w = 75.0_f32;
            let card_h = 80.0_f32;
            let item_spacing = 12.0_f32;

            let num_cols = ((avail_w + item_spacing) / (card_w + item_spacing))
                .floor()
                .max(1.0) as usize;

            egui::ScrollArea::vertical().show(ui, |ui| {
                let mut sorted_keys: Vec<_> = self.alphabet.keys().cloned().collect();
                sorted_keys.sort();

                egui::Grid::new("alphabet_preview_grid")
                    .spacing([item_spacing, item_spacing])
                    .show(ui, |ui| {
                        let mut col_count = 0;

                        for ch in sorted_keys {
                            let active_boxes = &self.alphabet[&ch];

                            ui.vertical_centered(|ui| {
                                let (response, painter) =
                                    ui.allocate_painter(vec2(card_w, card_h), Sense::hover());

                                // Card Background: White Card with Bold Dark Border
                                painter.rect_filled(
                                    response.rect,
                                    Rounding::same(6.0_f32),
                                    Color32::WHITE,
                                );
                                painter.rect_stroke(
                                    response.rect,
                                    Rounding::same(6.0_f32),
                                    Stroke::new(2.0_f32, Color32::from_rgb(20, 20, 25)),
                                );

                                let total_w: f32 =
                                    self.col_widths.iter().sum::<f32>() + (self.gap * 2.0);
                                let total_h: f32 =
                                    self.row_heights.iter().sum::<f32>() + (self.gap * 4.0);

                                let scale = (55.0 / total_w).min(60.0 / total_h);

                                let scaled_w = total_w * scale;
                                let card_center = response.rect.center();

                                let card_origin = pos2(
                                    card_center.x - (scaled_w / 2.0),
                                    response.rect.min.y + 8.0,
                                );

                                let mut box_idx = 1;
                                let mut curr_y = card_origin.y;

                                for r in 0..5 {
                                    let mut curr_x = card_origin.x;
                                    for c in 0..3 {
                                        let w = self.col_widths[c] * scale;
                                        let h = self.row_heights[r] * scale;

                                        if active_boxes.contains(&box_idx) {
                                            let rect = Rect::from_min_size(
                                                pos2(curr_x, curr_y),
                                                vec2(w, h),
                                            );
                                            // Fill with High-Contrast Orange
                                            painter.rect_filled(
                                                rect,
                                                Rounding::same(self.corner_radius * scale),
                                                orange,
                                            );
                                        }

                                        curr_x += w + (self.gap * scale);
                                        box_idx += 1;
                                    }
                                    curr_y += (self.row_heights[r] * scale) + (self.gap * scale);
                                }

                                ui.add_space(2.0);
                                ui.label(
                                    egui::RichText::new(ch.to_string())
                                        .strong()
                                        .color(Color32::from_rgb(15, 15, 18)),
                                );
                            });

                            col_count += 1;
                            if col_count >= num_cols {
                                ui.end_row();
                                col_count = 0;
                            }
                        }
                    });
            });
        });
    }
}
