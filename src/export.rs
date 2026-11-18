use crate::{app::Design, geometry, truetype};
use image::{Rgba, RgbaImage};
use std::{fs, path::Path};

pub struct ExportManager;
impl ExportManager {
    pub fn export_ttf(d: &Design, path: &Path) -> Result<(), String> {
        d.validate()?;
        fs::write(path, truetype::build(d)?).map_err(|e| e.to_string())
    }
    pub fn export_pngs(d: &Design, dir: &Path) -> Result<usize, String> {
        d.validate()?;
        fs::create_dir_all(dir).map_err(|e| e.to_string())?;
        let scale = 512.0 / d.height();
        let padding = 16.0;
        let width = (d.visual_width() * scale + padding * 2.0).ceil() as u32;
        let height = 544;
        for &ch in d.alphabet.keys() {
            let mut img = RgbaImage::new(width, height);
            for poly in geometry::outlines(d, ch) {
                let points: Vec<_> = poly
                    .iter()
                    .map(|p| [p[0] * scale + padding, p[1] * scale + padding])
                    .collect();
                let minx = points
                    .iter()
                    .map(|p| p[0])
                    .fold(f32::INFINITY, f32::min)
                    .floor()
                    .max(0.0) as u32;
                let maxx = points.iter().map(|p| p[0]).fold(0.0, f32::max).ceil() as u32;
                let miny = points
                    .iter()
                    .map(|p| p[1])
                    .fold(f32::INFINITY, f32::min)
                    .floor()
                    .max(0.0) as u32;
                let maxy = points.iter().map(|p| p[1]).fold(0.0, f32::max).ceil() as u32;
                for y in miny..maxy.min(height) {
                    for x in minx..maxx.min(width) {
                        let mut coverage = 0;
                        for sy in 0..4 {
                            for sx in 0..4 {
                                if inside(
                                    &points,
                                    x as f32 + (sx as f32 + 0.5) / 4.0,
                                    y as f32 + (sy as f32 + 0.5) / 4.0,
                                ) {
                                    coverage += 1;
                                }
                            }
                        }
                        let alpha = (coverage * 255 / 16) as u8;
                        let old = img.get_pixel(x, y)[3];
                        img.put_pixel(x, y, Rgba([0, 0, 0, old.saturating_add(alpha)]));
                    }
                }
            }
            img.save(dir.join(format!("U+{:04X}.png", ch as u32)))
                .map_err(|e| e.to_string())?;
        }
        Ok(d.alphabet.len())
    }
    pub fn export_svg(d: &Design, path: &Path) -> Result<(), String> {
        d.validate()?;
        let mut keys: Vec<_> = d.alphabet.keys().copied().collect();
        keys.sort_unstable();
        let cell_w = d.visual_width() + 40.0;
        let cell_h = d.height() + 55.0;
        let mut svg = format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {} {}\">\n",
            cell_w * 8.0,
            cell_h * keys.len().div_ceil(8) as f32
        );
        for (i, ch) in keys.iter().enumerate() {
            let x = (i % 8) as f32 * cell_w + 20.0;
            let y = (i / 8) as f32 * cell_h + 10.0;
            svg.push_str(&format!(
                "<g transform=\"translate({x} {y})\" fill=\"black\">"
            ));
            for p in geometry::outlines(d, *ch) {
                svg.push_str("<polygon points=\"");
                for v in p {
                    svg.push_str(&format!("{:.3},{:.3} ", v[0], v[1]));
                }
                svg.push_str("\"/>");
            }
            svg.push_str(&format!(
                "<text y=\"{}\" font-size=\"16\">U+{:04X}</text></g>\n",
                d.height() + 25.0,
                *ch as u32
            ));
        }
        svg.push_str("</svg>\n");
        fs::write(path, svg).map_err(|e| e.to_string())
    }
}
fn inside(p: &[[f32; 2]], x: f32, y: f32) -> bool {
    let mut sign = 0.0_f32;
    for i in 0..p.len() {
        let a = p[i];
        let b = p[(i + 1) % p.len()];
        let cross = (b[0] - a[0]) * (y - a[1]) - (b[1] - a[1]) * (x - a[0]);
        if cross.abs() > 0.0001 {
            if sign != 0.0 && cross.signum() != sign {
                return false;
            }
            sign = cross.signum();
        }
    }
    true
}
