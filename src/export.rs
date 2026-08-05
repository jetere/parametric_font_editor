use crate::app::FontEditorApp;
use image::{Rgba, RgbaImage};
use std::fs;
use std::path::PathBuf;

pub struct ExportManager;

impl ExportManager {
    /// Export high-resolution PNG previews for all characters (A-Z)
    pub fn export_pngs(app: &FontEditorApp, output_dir: &PathBuf) -> Result<usize, String> {
        fs::create_dir_all(output_dir).map_err(|e| e.to_string())?;

        let scale = 10; // High resolution multiplier (e.g. 10x)
        let total_w =
            ((app.col_widths.iter().sum::<f32>() + (app.gap * 2.0)) * scale as f32) as u32;
        let total_h =
            ((app.row_heights.iter().sum::<f32>() + (app.gap * 4.0)) * scale as f32) as u32;

        let padding = 20u32;
        let img_w = total_w + (padding * 2);
        let img_h = total_h + (padding * 2);

        let mut count = 0;

        for (&ch, active_boxes) in &app.alphabet {
            let mut img = RgbaImage::from_pixel(img_w, img_h, Rgba([24, 24, 28, 255]));

            let corner_r = (app.corner_radius * scale as f32) as i32;

            let mut box_idx = 1;
            let mut curr_y = padding as f32;

            for r in 0..5 {
                let mut curr_x = padding as f32;
                for c in 0..3 {
                    let w = app.col_widths[c] * scale as f32;
                    let h = app.row_heights[r] * scale as f32;

                    if active_boxes.contains(&box_idx) {
                        let x0 = curr_x as u32;
                        let y0 = curr_y as u32;
                        let x1 = (curr_x + w) as u32;
                        let y1 = (curr_y + h) as u32;

                        // Draw filled rectangle on image pixel buffer
                        for px in x0..x1.min(img_w) {
                            for py in y0..y1.min(img_h) {
                                // Simple corner radius rounding check
                                let dx = if px < x0 + corner_r as u32 {
                                    (x0 + corner_r as u32) as i32 - px as i32
                                } else if px > x1.saturating_sub(corner_r as u32) {
                                    px as i32 - (x1 - corner_r as u32) as i32
                                } else {
                                    0
                                };

                                let dy = if py < y0 + corner_r as u32 {
                                    (y0 + corner_r as u32) as i32 - py as i32
                                } else if py > y1.saturating_sub(corner_r as u32) {
                                    py as i32 - (y1 - corner_r as u32) as i32
                                } else {
                                    0
                                };

                                if dx * dx + dy * dy <= corner_r * corner_r {
                                    img.put_pixel(px, py, Rgba([129, 140, 248, 255]));
                                }
                            }
                        }
                    }

                    curr_x += w + (app.gap * scale as f32);
                    box_idx += 1;
                }
                curr_y += (app.row_heights[r] * scale as f32) + (app.gap * scale as f32);
            }

            let file_path = output_dir.join(format!("{}.png", ch));
            img.save(&file_path).map_err(|e| e.to_string())?;
            count += 1;
        }

        Ok(count)
    }

    /// Export SVG font package (usable on Windows and convertible to TTF)
    pub fn export_svg_font(app: &FontEditorApp, output_file: &PathBuf) -> Result<(), String> {
        let total_w = app.col_widths.iter().sum::<f32>() + (app.gap * 2.0);
        let total_h = app.row_heights.iter().sum::<f32>() + (app.gap * 4.0);

        let mut svg_content = format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg">
  <defs>
    <font id="ParametricFont" horiz-adv-x="{}">
      <font-face units-per-em="1000" ascent="800" descent="-200" />
      <missing-glyph horiz-adv-x="500" d="" />
"#,
            (total_w * 5.0) as u32
        );

        for (&ch, active_boxes) in &app.alphabet {
            let mut paths = String::new();
            let mut box_idx = 1;
            let mut curr_y = 0.0_f32;

            for r in 0..5 {
                let mut curr_x = 0.0_f32;
                for c in 0..3 {
                    let w = app.col_widths[c] * 5.0;
                    let h = app.row_heights[r] * 5.0;

                    if active_boxes.contains(&box_idx) {
                        paths.push_str(&format!(
                            "M {:.1} {:.1} h {:.1} v {:.1} h -{:.1} Z ",
                            curr_x,
                            total_h * 5.0 - curr_y,
                            w,
                            -h,
                            w
                        ));
                    }

                    curr_x += w + (app.gap * 5.0);
                    box_idx += 1;
                }
                curr_y += (app.row_heights[r] * 5.0) + (app.gap * 5.0);
            }

            svg_content.push_str(&format!(
                r#"      <glyph unicode="{}" glyph-name="{}" horiz-adv-x="{}" d="{}" />
"#,
                ch,
                ch,
                (total_w * 5.0) as u32,
                paths
            ));
        }

        svg_content.push_str(
            r#"    </font>
  </defs>
</svg>"#,
        );

        fs::write(output_file, svg_content).map_err(|e| e.to_string())
    }
}
