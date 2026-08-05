mod alphabet;
mod app;
mod canvas;
mod export;

use app::FontEditorApp;
use eframe::egui;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1150.0, 720.0])
            .with_title("paramatic_font_editor"),
        ..Default::default()
    };

    eframe::run_native(
        "Parametric Font Studio",
        native_options,
        Box::new(|_cc| Box::new(FontEditorApp::default())),
    )
}
