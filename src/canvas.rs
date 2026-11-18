use crate::app::FontEditorApp;
use eframe::egui::{self, pos2, vec2, Color32, Rect, Sense, Stroke};

pub fn render_vector_canvas(ui: &mut egui::Ui, app: &mut FontEditorApp) {
    ui.heading(format!("Edit character: {}", app.selected));
    ui.label("Click cells to draw. Drag orange nodes to resize rows and columns.");
    let (response, painter) =
        ui.allocate_painter(vec2(ui.available_width(), 260.0), Sense::hover());
    painter.rect_filled(response.rect, 6.0, Color32::from_rgb(24, 27, 34));
    let d = &mut app.design;
    let scale = ((response.rect.width() - 42.0) / d.width())
        .min((response.rect.height() - 42.0) / d.height())
        .min(1.0);
    let origin = response.rect.center() - vec2(d.width(), d.height()) * scale / 2.0;
    let mut xs = vec![origin.x];
    let mut ys = vec![origin.y];
    for w in d.col_widths {
        xs.push(xs.last().unwrap() + (w + d.gap) * scale);
    }
    for h in d.row_heights {
        ys.push(ys.last().unwrap() + (h + d.gap) * scale);
    }
    for r in 0..5 {
        for c in 0..3 {
            let index = r * 3 + c + 1;
            let rect = Rect::from_min_size(
                pos2(xs[c], ys[r]),
                vec2(d.col_widths[c], d.row_heights[r]) * scale,
            );
            let active = d
                .alphabet
                .get(&app.selected)
                .is_some_and(|v| v.contains(&index));
            let hit = ui.interact(
                rect.shrink(2.0),
                ui.id().with(("cell", index)),
                Sense::click(),
            );
            if hit.clicked() {
                d.set_cell(app.selected, index, !active, app.mirror_draw);
            }
            painter.rect_filled(
                rect,
                d.corner_radius
                    .min(d.col_widths[c] / 2.0)
                    .min(d.row_heights[r] / 2.0)
                    * scale,
                if active {
                    Color32::WHITE
                } else {
                    Color32::from_gray(45)
                },
            );
            if app.show_grid {
                painter.rect_stroke(rect, 0.0, Stroke::new(1.0_f32, Color32::from_gray(95)));
            }
        }
    }
    for c in 0..2 {
        for r in 0..4 {
            let p = pos2(
                xs[c + 1] - d.gap * scale / 2.0,
                ys[r + 1] - d.gap * scale / 2.0,
            );
            let hit = ui.interact(
                Rect::from_center_size(p, vec2(12.0, 12.0)),
                ui.id().with(("node", c, r)),
                Sense::drag(),
            );
            if hit.dragged() {
                let delta = ui.input(|i| i.pointer.delta()) / scale;
                d.col_widths[c] = (d.col_widths[c] + delta.x).clamp(10.0, 120.0);
                d.row_heights[r] = (d.row_heights[r] + delta.y).clamp(10.0, 100.0);
            }
            painter.circle_filled(p, 4.0, Color32::from_rgb(255, 140, 45));
        }
    }
    ui.small("Cell view is unslanted. The previews show the final shape.");
}
