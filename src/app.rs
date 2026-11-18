use crate::{
    alphabet::default_alphabet, canvas::render_vector_canvas, export::ExportManager, geometry,
};
use eframe::egui::{self, pos2, vec2, Color32, Sense};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Design {
    pub col_widths: [f32; 3],
    pub row_heights: [f32; 5],
    pub gap: f32,
    pub corner_radius: f32,
    pub connect_cells: bool,
    pub bridge_width: f32,
    pub slant: f32,
    pub spacing: f32,
    pub family: String,
    pub alphabet: HashMap<char, Vec<usize>>,
}
impl Default for Design {
    fn default() -> Self {
        Self {
            col_widths: [45.0; 3],
            row_heights: [40.0; 5],
            gap: 0.0,
            corner_radius: 0.0,
            connect_cells: false,
            bridge_width: 0.35,
            slant: 0.0,
            spacing: 22.0,
            family: "My Parametric Font".into(),
            alphabet: default_alphabet(),
        }
    }
}
impl Design {
    pub fn preset(&mut self, name: &str) {
        let (gap, radius, connected) = match name {
            "Soft" => (0.0, 12.0, true),
            "Stencil" => (8.0, 0.0, false),
            "Linked" => (14.0, 16.0, true),
            _ => (0.0, 0.0, false),
        };
        self.gap = gap;
        self.corner_radius = radius;
        self.connect_cells = connected;
        self.bridge_width = 0.35;
        self.slant = 0.0;
    }
    pub fn shift_glyph(&mut self, ch: char, dx: i32, dy: i32) {
        if let Some(cells) = self.alphabet.get_mut(&ch) {
            for cell in cells.iter_mut() {
                let n = *cell as i32 - 1;
                *cell = ((n / 3 + dy).rem_euclid(5) * 3 + (n % 3 + dx).rem_euclid(3) + 1) as usize;
            }
            cells.sort_unstable();
        }
    }
    pub fn set_cell(&mut self, ch: char, index: usize, active: bool, mirror: bool) {
        let cells = self.alphabet.entry(ch).or_default();
        let n = index - 1;
        let other = n / 3 * 3 + 2 - n % 3 + 1;
        for i in [index, if mirror { other } else { index }] {
            cells.retain(|&v| v != i);
            if active {
                cells.push(i);
            }
        }
        cells.sort_unstable();
    }
    pub fn width(&self) -> f32 {
        self.col_widths.iter().sum::<f32>() + self.gap * 2.0
    }
    pub fn height(&self) -> f32 {
        self.row_heights.iter().sum::<f32>() + self.gap * 4.0
    }
    pub fn visual_width(&self) -> f32 {
        self.width() + self.slant.abs() * self.height()
    }
    pub fn validate(&self) -> Result<(), String> {
        let valid = |v: f32, lo: f32, hi: f32| v.is_finite() && v >= lo && v <= hi;
        if !self.col_widths.iter().all(|&v| valid(v, 10.0, 120.0))
            || !self.row_heights.iter().all(|&v| valid(v, 10.0, 100.0))
            || !valid(self.gap, 0.0, 20.0)
            || !valid(self.corner_radius, 0.0, 25.0)
            || !valid(self.bridge_width, 0.1, 1.0)
            || !valid(self.slant, -0.3, 0.3)
            || !valid(self.spacing, 4.0, 80.0)
            || self.family.trim().is_empty()
            || self.family.chars().count() > 60
            || self.alphabet.len() > 128
            || self.alphabet.iter().any(|(c, v)| {
                !c.is_ascii()
                    || c.is_ascii_control()
                    || v.len() > 15
                    || v.iter().any(|i| !(1..=15).contains(i))
            })
        {
            return Err("Invalid project: check font name, dimensions and character cells.".into());
        }
        Ok(())
    }
    pub fn set_size(&mut self, w: f32, h: f32) {
        let sx = (w - self.gap * 2.0) / self.col_widths.iter().sum::<f32>();
        let sy = (h - self.gap * 4.0) / self.row_heights.iter().sum::<f32>();
        if self
            .col_widths
            .iter()
            .all(|v| (10.0..=120.0).contains(&(v * sx)))
            && self
                .row_heights
                .iter()
                .all(|v| (10.0..=100.0).contains(&(v * sy)))
        {
            for v in &mut self.col_widths {
                *v *= sx;
            }
            for v in &mut self.row_heights {
                *v *= sy;
            }
        }
    }
}

pub struct FontEditorApp {
    pub design: Design,
    pub selected: char,
    status: String,
    sample: String,
    preview_size: f32,
    locked: bool,
    undo: Vec<Design>,
    redo: Vec<Design>,
    gesture: Option<Design>,
    clipboard: Option<Vec<usize>>,
    pub mirror_draw: bool,
    pub show_grid: bool,
    comparison: Option<Design>,
}
impl Default for FontEditorApp {
    fn default() -> Self {
        Self {
            design: Design::default(),
            selected: 'A',
            status: String::new(),
            sample: "THE QUICK BROWN FOX\nSphinx of black quartz 0123456789".into(),
            preview_size: 48.0,
            locked: false,
            undo: Vec::new(),
            redo: Vec::new(),
            gesture: None,
            clipboard: None,
            mirror_draw: false,
            show_grid: true,
            comparison: None,
        }
    }
}
impl FontEditorApp {
    fn undo(&mut self) {
        if let Some(d) = self.undo.pop() {
            self.redo.push(std::mem::replace(&mut self.design, d));
        }
    }
    fn redo(&mut self) {
        if let Some(d) = self.redo.pop() {
            self.undo.push(std::mem::replace(&mut self.design, d));
        }
    }
    fn files(&mut self, ui: &mut egui::Ui) {
        ui.heading("Save & export");
        ui.label("Font family name");
        ui.text_edit_singleline(&mut self.design.family);
        ui.horizontal(|ui| {
            if ui.button("Save project").clicked() {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("Font project", &["json"])
                    .set_file_name("font-project.json")
                    .save_file()
                {
                    self.status = match self
                        .design
                        .validate()
                        .and_then(|_| {
                            serde_json::to_string_pretty(&self.design).map_err(|e| e.to_string())
                        })
                        .and_then(|s| std::fs::write(path, s).map_err(|e| e.to_string()))
                    {
                        Ok(()) => "Project saved.".into(),
                        Err(e) => e,
                    };
                }
            }
            if ui.button("Open project").clicked() {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("Font project", &["json"])
                    .pick_file()
                {
                    let result = (|| {
                        let bytes = std::fs::read(path).map_err(|e| e.to_string())?;
                        if bytes.len() > 1_000_000 {
                            return Err("Project is too large.".into());
                        }
                        let d: Design =
                            serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
                        d.validate()?;
                        Ok(d)
                    })();
                    match result {
                        Ok(d) => {
                            self.design = d;
                            self.status = "Project opened.".into();
                        }
                        Err(e) => self.status = e,
                    }
                }
            }
        });
        if ui.button("Export installable font (.ttf)").clicked() {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("TrueType font", &["ttf"])
                .set_file_name("MyParametricFont.ttf")
                .save_file()
            {
                self.status=match ExportManager::export_ttf(&self.design,&path) { Ok(())=>"TTF saved. Open it in Windows and choose Install; then select your font family in an app.".into(),Err(e)=>format!("Export failed: {e}") };
            }
        }
        ui.horizontal(|ui| {
            if ui.button("PNG previews").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    self.status = match ExportManager::export_pngs(&self.design, &path) {
                        Ok(n) => format!("Saved {n} PNG previews."),
                        Err(e) => e,
                    };
                }
            }
            if ui.button("SVG specimen").clicked() {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("SVG artwork", &["svg"])
                    .set_file_name("specimen.svg")
                    .save_file()
                {
                    self.status = match ExportManager::export_svg(&self.design, &path) {
                        Ok(()) => "SVG specimen saved.".into(),
                        Err(e) => e,
                    };
                }
            }
        });
        ui.small("TTF installs as a system font. JSON preserves editable source. SVG is artwork.");
        ui.label(&self.status);
    }
}
impl eframe::App for FontEditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::light());
        let mut history_action = false;
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Parametric Font Studio");
                if ui
                    .add_enabled(!self.undo.is_empty(), egui::Button::new("Undo"))
                    .clicked()
                {
                    self.undo();
                    history_action = true;
                }
                if ui
                    .add_enabled(!self.redo.is_empty(), egui::Button::new("Redo"))
                    .clicked()
                {
                    self.redo();
                    history_action = true;
                }
                ui.label("3 x 5 pixel typeface designer");
            });
        });
        let before = self.design.clone();
        egui::SidePanel::left("controls")
            .default_width(370.0)
            .resizable(true)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical()
                    .id_source("controls_scroll")
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.checkbox(&mut self.mirror_draw, "Mirror drawing");
                            ui.checkbox(&mut self.show_grid, "Grid lines");
                        });
                        render_vector_canvas(ui, self);
                        ui.horizontal(|ui| {
                            ui.label("Move glyph:");
                            for (label, dx, dy) in [("Left", -1, 0), ("Right", 1, 0), ("Up", 0, -1), ("Down", 0, 1)] {
                                if ui.button(label).on_hover_text("Wrap cells around the grid edge").clicked() {
                                    self.design.shift_glyph(self.selected, dx, dy);
                                }
                            }
                        });
                        ui.horizontal(|ui| {
                            if ui.button("Clear").clicked() {
                                self.design.alphabet.insert(self.selected, vec![]);
                            }
                            if ui.button("Reset glyph").clicked() {
                                self.design.alphabet.insert(
                                    self.selected,
                                    default_alphabet()
                                        .get(&self.selected)
                                        .cloned()
                                        .unwrap_or_default(),
                                );
                            }
                            if ui.button("Copy").clicked() {
                                self.clipboard = self.design.alphabet.get(&self.selected).cloned();
                            }
                            if ui
                                .add_enabled(self.clipboard.is_some(), egui::Button::new("Paste"))
                                .clicked()
                            {
                                self.design
                                    .alphabet
                                    .insert(self.selected, self.clipboard.clone().unwrap());
                            }
                        });
                        ui.horizontal(|ui| {
                            if ui.button("Flip horizontal").clicked() {
                                if let Some(v) = self.design.alphabet.get_mut(&self.selected) {
                                    for i in v {
                                        let n = *i - 1;
                                        *i = n / 3 * 3 + 2 - n % 3 + 1;
                                    }
                                }
                            }
                            if ui.button("Flip vertical").clicked() {
                                if let Some(v) = self.design.alphabet.get_mut(&self.selected) {
                                    for i in v {
                                        let n = *i - 1;
                                        *i = (4 - n / 3) * 3 + n % 3 + 1;
                                    }
                                }
                            }
                        });
                        ui.separator();
                        ui.heading("Global geometry");
                        ui.small("These controls affect the entire typeface.");
                        ui.horizontal_wrapped(|ui| {
                            ui.label("Style presets:");
                            for name in ["Pixel", "Soft", "Stencil", "Linked"] {
                                if ui.button(name).clicked() { self.design.preset(name); }
                            }
                        });
                        let (mut w, mut h) = (self.design.width(), self.design.height());
                        let (ow, oh) = (w, h);
                        ui.horizontal(|ui| {
                            let wc = ui
                                .add(
                                    egui::DragValue::new(&mut w)
                                        .clamp_range(30.0..=400.0)
                                        .prefix("W "),
                                )
                                .changed();
                            let hc = ui
                                .add(
                                    egui::DragValue::new(&mut h)
                                        .clamp_range(50.0..=580.0)
                                        .prefix("H "),
                                )
                                .changed();
                            if wc {
                                if self.locked {
                                    h = oh * w / ow;
                                }
                                self.design.set_size(w, h);
                            } else if hc {
                                if self.locked {
                                    w = ow * h / oh;
                                }
                                self.design.set_size(w, h);
                            }
                            ui.checkbox(&mut self.locked, "Lock ratio");
                        });
                        ui.horizontal(|ui| {
                            if ui.button("Square 1:1").clicked() {
                                self.design.set_size(220.0, 220.0);
                            }
                            if ui.button("Tall 4:5").clicked() {
                                self.design.set_size(176.0, 220.0);
                            }
                            if ui.button("Reset style").clicked() {
                                let alphabet = self.design.alphabet.clone();
                                let family = self.design.family.clone();
                                self.design = Design {
                                    alphabet,
                                    family,
                                    ..Design::default()
                                };
                            }
                        });
                        ui.add(
                            egui::Slider::new(&mut self.design.gap, 0.0..=20.0).text("Cell gap"),
                        );
                        ui.add(
                            egui::Slider::new(&mut self.design.corner_radius, 0.0..=25.0)
                                .text("Roundness"),
                        );
                        ui.checkbox(&mut self.design.connect_cells, "Connect neighboring cells");
                        if self.design.connect_cells {
                            ui.add(egui::Slider::new(&mut self.design.bridge_width, 0.1..=1.0).text("Connector thickness"));
                            ui.small("Bridges keep adjacent active cells joined, even with gaps and rounded edges.");
                        }
                        ui.add(egui::Slider::new(&mut self.design.slant, -0.3..=0.3).text("Slant"));
                        ui.add(
                            egui::Slider::new(&mut self.design.spacing, 4.0..=80.0)
                                .text("Letter spacing"),
                        );
                        ui.collapsing("Individual row and column sizes", |ui| {
                            for (i, v) in self.design.col_widths.iter_mut().enumerate() {
                                ui.add(
                                    egui::Slider::new(v, 10.0..=120.0)
                                        .text(format!("Column {}", i + 1)),
                                );
                            }
                            for (i, v) in self.design.row_heights.iter_mut().enumerate() {
                                ui.add(
                                    egui::Slider::new(v, 10.0..=100.0)
                                        .text(format!("Row {}", i + 1)),
                                );
                            }
                        });
                        ui.separator();
                        self.files(ui);
                    });
            });
        egui::CentralPanel::default().show(ctx,|ui|{
            egui::ScrollArea::vertical().id_source("previews").show(ui,|ui|{
                ui.heading("Live text preview");
                ui.horizontal(|ui| {
                    if ui.button("Capture comparison").clicked() { self.comparison = Some(self.design.clone()); }
                    if ui.add_enabled(self.comparison.is_some(), egui::Button::new("Restore captured design")).clicked() {
                        self.design = self.comparison.clone().unwrap();
                    }
                    if ui.button("Dismiss comparison").clicked() { self.comparison = None; }
                });
                if let Some(saved) = &self.comparison {
                    ui.label(format!("Comparison for {}: captured / current", self.selected));
                    ui.horizontal(|ui| {
                        for d in [saved, &self.design] {
                            let (res,p) = ui.allocate_painter(vec2(140.0,140.0),Sense::hover());
                            p.rect_filled(res.rect,4.0,Color32::WHITE);
                            geometry::paint_fitted(&p,d,self.selected,res.rect.shrink(10.0),Color32::BLACK);
                        }
                    });
                }
                ui.add(egui::TextEdit::multiline(&mut self.sample).char_limit(2000).desired_width(f32::INFINITY));
                ui.add(egui::Slider::new(&mut self.preview_size,16.0..=120.0).text("Preview height"));
                let scale=self.preview_size/self.design.height();
                let advance=(self.design.visual_width()+self.design.spacing)*scale;
                let columns=(ui.available_width()/advance).floor().max(1.0) as usize;
                let rows=self.sample.lines().map(|s|s.chars().count().max(1).div_ceil(columns)).sum::<usize>().max(1);
                let (res,p)=ui.allocate_painter(vec2(ui.available_width(),rows as f32*(self.preview_size+14.0)),Sense::hover());
                let mut col=0;let mut row=0;
                for ch in self.sample.chars().take(2000) {
                    if ch=='\n' {row+=1;col=0;continue;}
                    if col==columns {row+=1;col=0;}
                    let origin=pos2(res.rect.left()+col as f32*advance,res.rect.top()+row as f32*(self.preview_size+14.0));
                    if self.design.alphabet.contains_key(&ch){geometry::paint(&p,&self.design,ch,origin,scale,Color32::BLACK);}
                    else if ch!=' ' {p.rect_stroke(egui::Rect::from_min_size(origin,vec2(advance-4.0,self.preview_size)),0.0,egui::Stroke::new(1.0_f32,Color32::RED));}
                    col+=1;
                }
                ui.separator();ui.heading("Characters — click to edit");
                ui.small("Uppercase, lowercase, digits and punctuation. Red boxes in text indicate unsupported characters.");
                let mut keys:Vec<_>=self.design.alphabet.keys().copied().collect();keys.sort_unstable();
                ui.horizontal_wrapped(|ui|{
                    for ch in keys {
                        ui.allocate_ui_with_layout(vec2(64.0,106.0),egui::Layout::top_down(egui::Align::Center),|ui|{
                            let (res,p)=ui.allocate_painter(vec2(64.0,82.0),Sense::click());
                            p.rect_filled(res.rect,4.0,if ch==self.selected {Color32::from_rgb(255,226,195)}else{Color32::WHITE});
                            geometry::paint_fitted(&p,&self.design,ch,res.rect.shrink(8.0),Color32::BLACK);
                            if res.clicked(){self.selected=ch;}
                            ui.label(if ch==' ' {"space".into()}else{ch.to_string()});
                        });
                    }
                });
            });
        });
        if !history_action && before != self.design {
            if self.gesture.is_none() {
                self.gesture = Some(before);
            }
            self.redo.clear();
        }
        if !ctx.input(|i| i.pointer.any_down()) {
            if let Some(d) = self.gesture.take() {
                if d != self.design {
                    self.undo.push(d);
                    if self.undo.len() > 100 {
                        self.undo.remove(0);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn size_presets_include_gaps() {
        let mut d = Design {
            gap: 12.0,
            ..Design::default()
        };
        d.set_size(220.0, 220.0);
        assert!((d.width() - d.height()).abs() < 0.001);
        d.set_size(176.0, 220.0);
        assert!((d.width() / d.height() - 0.8).abs() < 0.001);
    }
    #[test]
    fn project_roundtrip_and_validation() {
        let d = Design::default();
        let s = serde_json::to_string(&d).unwrap();
        let mut loaded: Design = serde_json::from_str(&s).unwrap();
        assert!(loaded == d);
        loaded.gap = f32::NAN;
        assert!(loaded.validate().is_err());
    }
}

#[cfg(test)]
mod feature_tests {
    use super::*;
    #[test]
    fn mirrors_and_shifts_without_losing_cells() {
        let mut d = Design::default();
        d.alphabet.insert('A', vec![]);
        d.set_cell('A', 1, true, true);
        assert_eq!(d.alphabet[&'A'], vec![1, 3]);
        d.set_cell('A', 2, true, true);
        assert_eq!(d.alphabet[&'A'], vec![1, 2, 3]);
        let original = d.alphabet[&'A'].clone();
        d.shift_glyph('A', -1, -1);
        d.shift_glyph('A', 1, 1);
        assert_eq!(d.alphabet[&'A'], original);
        d.set_cell('A', 1, false, true);
        assert_eq!(d.alphabet[&'A'], vec![2]);
    }
    #[test]
    fn old_projects_get_defaults_and_new_settings_roundtrip() {
        let mut json = serde_json::to_value(Design::default()).unwrap();
        json.as_object_mut().unwrap().remove("connect_cells");
        json.as_object_mut().unwrap().remove("bridge_width");
        let mut d: Design = serde_json::from_value(json).unwrap();
        assert!(!d.connect_cells);
        let glyphs = d.alphabet.clone();
        d.preset("Linked");
        assert_eq!(d.alphabet, glyphs);
        let saved = serde_json::to_string(&d).unwrap();
        let loaded: Design = serde_json::from_str(&saved).unwrap();
        assert!(d == loaded);
        d.bridge_width = 0.0;
        assert!(d.validate().is_err());
    }
}
