use crate::app::FontEditorApp;
use eframe::egui;
use egui::{pos2, vec2, Color32, Rect, Rounding, Sense, Stroke};

pub fn render_vector_canvas(ui: &mut egui::Ui, app: &mut FontEditorApp) {
    ui.heading("Vector Canvas Editor");
    ui.label("Drag internal vertex nodes or any 4 outer edges directly:");
    ui.add_space(6.0);

    // 1. Parameter Clamping Limits
    for w in app.col_widths.iter_mut() {
        *w = w.clamp(10.0, 120.0);
    }
    for h in app.row_heights.iter_mut() {
        *h = h.clamp(10.0, 100.0);
    }
    app.gap = app.gap.clamp(0.0, 20.0);
    app.corner_radius = app.corner_radius.clamp(0.0, 25.0);

    let total_grid_w: f32 = app.col_widths.iter().sum::<f32>() + (app.gap * 2.0);
    let total_grid_h: f32 = app.row_heights.iter().sum::<f32>() + (app.gap * 4.0);

    let canvas_size = vec2(380.0, 320.0);
    let (response, painter) = ui.allocate_painter(canvas_size, Sense::hover());

    // Stark Black Artboard Background with Bold White Border
    painter.rect_filled(response.rect, Rounding::same(4.0_f32), Color32::BLACK);
    painter.rect_stroke(
        response.rect,
        Rounding::same(4.0_f32),
        Stroke::new(2.5_f32, Color32::WHITE),
    );

    // Centering Anchor Logic
    let canvas_center = response.rect.center();
    let origin = pos2(
        canvas_center.x - (total_grid_w / 2.0),
        canvas_center.y - (total_grid_h / 2.0),
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

    // Render 15 Grid Rectangles with Bold White Stroke
    for r in 0..5 {
        for c in 0..3 {
            let rect = Rect::from_min_size(
                pos2(col_x[c], row_y[r]),
                vec2(app.col_widths[c], app.row_heights[r]),
            );
            painter.rect_stroke(
                rect,
                Rounding::same(app.corner_radius),
                Stroke::new(2.0_f32, Color32::WHITE),
            );
        }
    }

    // High-Contrast Orange Color Definition
    let orange = Color32::from_rgb(255, 102, 0);

    // --- 4 EXTERIOR EDGE DRAG HANDLES ---

    // Left Border Handle
    let left_edge_rect = Rect::from_min_max(
        pos2(col_x[0] - 6.0, row_y[0]),
        pos2(col_x[0] + 6.0, row_y[5]),
    );
    let left_resp = ui.interact(
        left_edge_rect,
        ui.make_persistent_id("ext_edge_left"),
        Sense::drag(),
    );
    if left_resp.dragged() {
        let delta = -left_resp.drag_delta().x;
        app.col_widths[0] = (app.col_widths[0] + delta).clamp(10.0, 120.0);
    }
    if left_resp.hovered() || left_resp.dragged() {
        painter.line_segment(
            [pos2(col_x[0], row_y[0]), pos2(col_x[0], row_y[5])],
            Stroke::new(3.5_f32, orange),
        );
    }

    // Right Border Handle
    let right_edge_rect = Rect::from_min_max(
        pos2(col_x[3] - 6.0, row_y[0]),
        pos2(col_x[3] + 6.0, row_y[5]),
    );
    let right_resp = ui.interact(
        right_edge_rect,
        ui.make_persistent_id("ext_edge_right"),
        Sense::drag(),
    );
    if right_resp.dragged() {
        let delta = right_resp.drag_delta().x;
        app.col_widths[2] = (app.col_widths[2] + delta).clamp(10.0, 120.0);
    }
    if right_resp.hovered() || right_resp.dragged() {
        painter.line_segment(
            [pos2(col_x[3], row_y[0]), pos2(col_x[3], row_y[5])],
            Stroke::new(3.5_f32, orange),
        );
    }

    // Top Border Handle
    let top_edge_rect = Rect::from_min_max(
        pos2(col_x[0], row_y[0] - 6.0),
        pos2(col_x[3], row_y[0] + 6.0),
    );
    let top_resp = ui.interact(
        top_edge_rect,
        ui.make_persistent_id("ext_edge_top"),
        Sense::drag(),
    );
    if top_resp.dragged() {
        let delta = -top_resp.drag_delta().y;
        app.row_heights[0] = (app.row_heights[0] + delta).clamp(10.0, 100.0);
    }
    if top_resp.hovered() || top_resp.dragged() {
        painter.line_segment(
            [pos2(col_x[0], row_y[0]), pos2(col_x[3], row_y[0])],
            Stroke::new(3.5_f32, orange),
        );
    }

    // Bottom Border Handle
    let bottom_edge_rect = Rect::from_min_max(
        pos2(col_x[0], row_y[5] - 6.0),
        pos2(col_x[3], row_y[5] + 6.0),
    );
    let bottom_resp = ui.interact(
        bottom_edge_rect,
        ui.make_persistent_id("ext_edge_bottom"),
        Sense::drag(),
    );
    if bottom_resp.dragged() {
        let delta = bottom_resp.drag_delta().y;
        app.row_heights[4] = (app.row_heights[4] + delta).clamp(10.0, 100.0);
    }
    if bottom_resp.hovered() || bottom_resp.dragged() {
        painter.line_segment(
            [pos2(col_x[0], row_y[5]), pos2(col_x[3], row_y[5])],
            Stroke::new(3.5_f32, orange),
        );
    }

    // --- INTERNAL EDGE DRAGGING ---
    for c in 0..2 {
        let edge_x = col_x[c + 1] - app.gap / 2.0;
        let edge_rect =
            Rect::from_min_max(pos2(edge_x - 4.0, row_y[0]), pos2(edge_x + 4.0, row_y[5]));
        let resp = ui.interact(
            edge_rect,
            ui.make_persistent_id(format!("edge_v_{}", c)),
            Sense::drag(),
        );
        if resp.dragged() {
            app.col_widths[c] = (app.col_widths[c] + resp.drag_delta().x).clamp(10.0, 120.0);
        }
        if resp.hovered() || resp.dragged() {
            painter.line_segment(
                [pos2(edge_x, row_y[0]), pos2(edge_x, row_y[5])],
                Stroke::new(2.5_f32, orange),
            );
        }
    }

    for r in 0..4 {
        let edge_y = row_y[r + 1] - app.gap / 2.0;
        let edge_rect =
            Rect::from_min_max(pos2(col_x[0], edge_y - 4.0), pos2(col_x[3], edge_y + 4.0));
        let resp = ui.interact(
            edge_rect,
            ui.make_persistent_id(format!("edge_h_{}", r)),
            Sense::drag(),
        );
        if resp.dragged() {
            app.row_heights[r] = (app.row_heights[r] + resp.drag_delta().y).clamp(10.0, 100.0);
        }
        if resp.hovered() || resp.dragged() {
            painter.line_segment(
                [pos2(col_x[0], edge_y), pos2(col_x[3], edge_y)],
                Stroke::new(2.5_f32, orange),
            );
        }
    }

    // --- 2D INTERIOR VERTEX ANCHOR HANDLES ---
    for c in 0..2 {
        for r in 0..4 {
            let vertex_pos = pos2(col_x[c + 1] - app.gap / 2.0, row_y[r + 1] - app.gap / 2.0);
            let handle_rect = Rect::from_center_size(vertex_pos, vec2(12.0, 12.0));
            let resp = ui.interact(
                handle_rect,
                ui.make_persistent_id(format!("vertex_{}_{}", c, r)),
                Sense::drag(),
            );

            if resp.dragged() {
                app.col_widths[c] = (app.col_widths[c] + resp.drag_delta().x).clamp(10.0, 120.0);
                app.row_heights[r] = (app.row_heights[r] + resp.drag_delta().y).clamp(10.0, 100.0);
            }

            let (fill_color, border_color) = if resp.hovered() || resp.dragged() {
                (Color32::WHITE, orange)
            } else {
                (orange, Color32::WHITE)
            };

            painter.rect_filled(handle_rect, Rounding::same(2.0_f32), fill_color);
            painter.rect_stroke(
                handle_rect,
                Rounding::same(2.0_f32),
                Stroke::new(2.0_f32, border_color),
            );
        }
    }
}
