use crate::app::Design;
use eframe::egui::{self, pos2, Color32, Pos2, Rect, Shape, Stroke};

/// One outline source for the canvas, PNG, SVG and TrueType exports.
pub fn outlines(d: &Design, ch: char) -> Vec<Vec<[f32; 2]>> {
    let Some(cells) = d.alphabet.get(&ch) else {
        return Vec::new();
    };
    let mut paths = Vec::new();
    let mut y = 0.0;
    for row in 0..5 {
        let mut x = 0.0;
        for col in 0..3 {
            let w = d.col_widths[col];
            let h = d.row_heights[row];
            if cells.contains(&(row * 3 + col + 1)) {
                let r = d.corner_radius.min(w / 2.0).min(h / 2.0);
                let mut points = Vec::new();
                if r < 0.01 {
                    points.extend([[x, y], [x + w, y], [x + w, y + h], [x, y + h]]);
                } else {
                    for (cx, cy, start) in [
                        (x + w - r, y + r, -90.0_f32),
                        (x + w - r, y + h - r, 0.0),
                        (x + r, y + h - r, 90.0),
                        (x + r, y + r, 180.0),
                    ] {
                        for step in 0..=6 {
                            let a = (start + step as f32 * 15.0).to_radians();
                            points.push([cx + r * a.cos(), cy + r * a.sin()]);
                        }
                    }
                }
                paths.push(points);
            }
            x += w + d.gap;
        }
        y += d.row_heights[row] + d.gap;
    }
    if d.connect_cells {
        let mut y = 0.0;
        for row in 0..5 {
            let mut x = 0.0;
            for col in 0..3 {
                let index = row * 3 + col + 1;
                let w = d.col_widths[col];
                let h = d.row_heights[row];
                if cells.contains(&index) {
                    if col < 2 && cells.contains(&(index + 1)) {
                        let thickness = h * d.bridge_width;
                        let cy = y + h / 2.0;
                        paths.push(rectangle(
                            x + w / 2.0,
                            cy - thickness / 2.0,
                            x + w + d.gap + d.col_widths[col + 1] / 2.0,
                            cy + thickness / 2.0,
                        ));
                    }
                    if row < 4 && cells.contains(&(index + 3)) {
                        let thickness = w * d.bridge_width;
                        let cx = x + w / 2.0;
                        paths.push(rectangle(
                            cx - thickness / 2.0,
                            y + h / 2.0,
                            cx + thickness / 2.0,
                            y + h + d.gap + d.row_heights[row + 1] / 2.0,
                        ));
                    }
                }
                x += w + d.gap;
            }
            y += d.row_heights[row] + d.gap;
        }
    }
    for outline in &mut paths {
        for p in outline {
            p[0] += d.slant * (d.height() - p[1]) + (-d.slant).max(0.0) * d.height();
        }
    }
    paths
}

fn rectangle(left: f32, top: f32, right: f32, bottom: f32) -> Vec<[f32; 2]> {
    vec![[left, top], [right, top], [right, bottom], [left, bottom]]
}

pub fn paint(
    painter: &egui::Painter,
    d: &Design,
    ch: char,
    origin: Pos2,
    scale: f32,
    color: Color32,
) {
    for outline in outlines(d, ch) {
        // Adjacent square cells share edges. Feathering each cell independently
        // creates pale seams through otherwise solid strokes.
        if d.gap == 0.0 && d.corner_radius == 0.0 {
            let mut mesh = egui::Mesh::default();
            for p in &outline {
                mesh.colored_vertex(
                    pos2(origin.x + p[0] * scale, origin.y + p[1] * scale),
                    color,
                );
            }
            for i in 1..outline.len() - 1 {
                mesh.add_triangle(0, i as u32, i as u32 + 1);
            }
            painter.add(Shape::mesh(mesh));
            continue;
        }
        painter.add(Shape::convex_polygon(
            outline
                .into_iter()
                .map(|p| pos2(origin.x + p[0] * scale, origin.y + p[1] * scale))
                .collect(),
            color,
            Stroke::NONE,
        ));
    }
}

pub fn paint_fitted(painter: &egui::Painter, d: &Design, ch: char, rect: Rect, color: Color32) {
    let scale = (rect.width() / d.visual_width()).min(rect.height() / d.height());
    paint(
        painter,
        d,
        ch,
        pos2(
            rect.center().x - d.visual_width() * scale / 2.0,
            rect.center().y - d.height() * scale / 2.0,
        ),
        scale,
        color,
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bridges_only_join_orthogonal_neighbors_and_span_gaps() {
        let mut d = Design {
            gap: 20.0,
            corner_radius: 20.0,
            connect_cells: true,
            ..Design::default()
        };
        d.alphabet.insert('A', vec![1, 2, 4, 9]);
        let paths = outlines(&d, 'A');
        assert_eq!(paths.len(), 6); // Four cells, one horizontal and one vertical bridge.
        let bridge = &paths[4];
        assert!(bridge[0][0] < d.col_widths[0]);
        assert!(bridge[1][0] > d.col_widths[0] + d.gap);
        for slant in [-0.3, 0.0, 0.3] {
            d.slant = slant;
            for p in outlines(&d, 'A').into_iter().flatten() {
                assert!(p[0] >= -0.001 && p[0] <= d.visual_width() + 0.001);
                assert!(p[1] >= -0.001 && p[1] <= d.height() + 0.001);
            }
        }
        d.connect_cells = false;
        assert_eq!(outlines(&d, 'A').len(), 4);
    }
}
