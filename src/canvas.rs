use crate::app::FontEditorApp;
use eframe::egui;
use egui::{pos2, vec2, Color32, Rect, Rounding, Sense, Stroke};

pub fn render_vector_canvas(ui: &mut egui::Ui, app: &mut FontEditorApp) {
    ui.heading("Vector Canvas Editor");
    ui.label("Drag interior vertex anchors or edges directly on the grid:");
    ui.add_space(8.0);

    let canvas_size = vec2(380.0, 340.0);
    let (response, painter) = ui.allocate_painter(canvas_size, Sense::hover());
    let origin = response.rect.min + vec2(20.0, 20.0);

    // Artboard Background
    painter.rect_filled(
        response.rect,
        Rounding::same(8.0_f32),
        Color32::from_rgb(18, 18, 20),
    );

    let mut col_x = [0.0_f32; 4];
    col_x[0] = origin.x;
    col_x[1] = col_x[0] + app.col_widths[0] + app.gap;
    col_x[2] = col_x[1] + app.col_widths[1] + app.gap;
    col_x[3] = col_x[2] + app.col_widths[2];

    let mut row_y = [0.0_f32; 6];
    row_y[0] = origin.y;
    for r in 0..5 {
        row_y[r + 1] = row_y[r] + app.row_heights[r] + app.gap;
    }

    // Grid Rectangles
    for r in 0..5 {
        for c in 0..3 {
            let rect = Rect::from_min_size(
                pos2(col_x[c], row_y[r]),
                vec2(app.col_widths[c], app.row_heights[r]),
            );
            painter.rect_stroke(
                rect,
                Rounding::same(app.corner_radius),
                Stroke::new(1.0_f32, Color32::from_rgb(60, 60, 75)),
            );
        }
    }

    // Vertical Internal Edge Dragging
    for c in 0..2 {
        let edge_x = col_x[c + 1] - app.gap / 2.0;
        let edge_rect =
            Rect::from_min_max(pos2(edge_x - 4.0, row_y[0]), pos2(edge_x + 4.0, row_y[5]));
        let id = ui.make_persistent_id(format!("edge_v_{}", c));
        let resp = ui.interact(edge_rect, id, Sense::drag());

        if resp.dragged() {
            app.col_widths[c] = (app.col_widths[c] + resp.drag_delta().x).max(15.0);
        }

        if resp.hovered() || resp.dragged() {
            painter.line_segment(
                [pos2(edge_x, row_y[0]), pos2(edge_x, row_y[5])],
                Stroke::new(2.0_f32, Color32::from_rgb(99, 102, 241)),
            );
        }
    }

    // Horizontal Internal Edge Dragging
    for r in 0..4 {
        let edge_y = row_y[r + 1] - app.gap / 2.0;
        let edge_rect =
            Rect::from_min_max(pos2(col_x[0], edge_y - 4.0), pos2(col_x[3], edge_y + 4.0));
        let id = ui.make_persistent_id(format!("edge_h_{}", r));
        let resp = ui.interact(edge_rect, id, Sense::drag());

        if resp.dragged() {
            app.row_heights[r] = (app.row_heights[r] + resp.drag_delta().y).max(15.0);
        }

        if resp.hovered() || resp.dragged() {
            painter.line_segment(
                [pos2(col_x[0], edge_y), pos2(col_x[3], edge_y)],
                Stroke::new(2.0_f32, Color32::from_rgb(99, 102, 241)),
            );
        }
    }

    // 2D Interior Vertex Anchor Handles
    for c in 0..2 {
        for r in 0..4 {
            let vertex_pos = pos2(col_x[c + 1] - app.gap / 2.0, row_y[r + 1] - app.gap / 2.0);
            let handle_rect = Rect::from_center_size(vertex_pos, vec2(12.0, 12.0));
            let id = ui.make_persistent_id(format!("vertex_{}_{}", c, r));
            let resp = ui.interact(handle_rect, id, Sense::drag());

            if resp.dragged() {
                app.col_widths[c] = (app.col_widths[c] + resp.drag_delta().x).max(15.0);
                app.row_heights[r] = (app.row_heights[r] + resp.drag_delta().y).max(15.0);
            }

            let (fill_color, border_color) = if resp.hovered() || resp.dragged() {
                (Color32::from_rgb(129, 140, 248), Color32::WHITE)
            } else {
                (
                    Color32::from_rgb(49, 46, 129),
                    Color32::from_rgb(165, 180, 252),
                )
            };

            painter.rect_filled(handle_rect, Rounding::same(2.0_f32), fill_color);
            painter.rect_stroke(
                handle_rect,
                Rounding::same(2.0_f32),
                Stroke::new(1.5_f32, border_color),
            );
        }
    }
}
