#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod alphabet;
mod app;
mod canvas;
mod export;
mod geometry;
mod truetype;

use app::FontEditorApp;
use eframe::egui;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1150.0, 720.0])
            .with_title("Parametric Font Studio"),
        ..Default::default()
    };

    eframe::run_native(
        "Parametric Font Studio",
        native_options,
        Box::new(|_cc| Box::new(FontEditorApp::default())),
    )
}
