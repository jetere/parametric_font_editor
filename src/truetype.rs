//! Minimal, unhinted TrueType writer. Coordinates use a 1000-unit em and 800-unit cap height.
use crate::{app::Design, geometry};
use std::collections::BTreeMap;
fn u16b(b: &mut Vec<u8>, v: u16) {
    b.extend(v.to_be_bytes());
}
fn i16b(b: &mut Vec<u8>, v: i16) {
    b.extend(v.to_be_bytes());
}
fn u32b(b: &mut Vec<u8>, v: u32) {
    b.extend(v.to_be_bytes());
}
fn set16(b: &mut [u8], at: usize, v: u16) {
    b[at..at + 2].copy_from_slice(&v.to_be_bytes());
}
fn set32(b: &mut [u8], at: usize, v: u32) {
    b[at..at + 4].copy_from_slice(&v.to_be_bytes());
}
fn checksum(b: &[u8]) -> u32 {
    b.chunks(4).fold(0u32, |sum, c| {
        let mut w = [0; 4];
        w[..c.len()].copy_from_slice(c);
        sum.wrapping_add(u32::from_be_bytes(w))
    })
}

pub fn build(d: &Design) -> Result<Vec<u8>, String> {
    d.validate()?;
    let mut keys: Vec<char> = d.alphabet.keys().copied().collect();
    keys.sort_unstable();
    let scale = 800.0 / d.height();
    let bearing = (d.spacing * scale / 2.0).round() as i16;
    let advance = ((d.visual_width() + d.spacing) * scale).ceil() as u16;
    let mut tables: BTreeMap<[u8; 4], Vec<u8>> = BTreeMap::new();
    let mut glyf = Vec::new();
    let mut loca = Vec::new();
    let mut hmtx = Vec::new();
    let mut max_points = 0;
    let mut max_contours = 0;
    let mut bounds = [i16::MAX, i16::MAX, i16::MIN, i16::MIN];
    let mut min_lsb = i16::MAX;
    let mut min_rsb = i16::MAX;
    let mut max_extent = i16::MIN;
    for ch in std::iter::once(None).chain(keys.iter().map(|&c| Some(c))) {
        u32b(&mut loca, glyf.len() as u32);
        let contours: Vec<Vec<(i16, i16)>> = match ch {
            Some(c) => geometry::outlines(d, c)
                .iter()
                .map(|p| {
                    p.iter()
                        .map(|v| {
                            (
                                (v[0] * scale).round() as i16 + bearing,
                                ((d.height() - v[1]) * scale).round() as i16,
                            )
                        })
                        .collect()
                })
                .collect(),
            // Visible missing-character box, with a counter wound in the opposite direction.
            None => vec![
                vec![(50, 0), (50, 700), (450, 700), (450, 0)],
                vec![(100, 50), (400, 50), (400, 650), (100, 650)],
            ],
        };
        let aw = if ch.is_none() { 500 } else { advance };
        let mut bb = [i16::MAX, i16::MAX, i16::MIN, i16::MIN];
        let points: Vec<_> = contours.iter().flatten().copied().collect();
        if points.is_empty() {
            bb = [0; 4];
        } else {
            for &(x, y) in &points {
                bb[0] = bb[0].min(x);
                bb[1] = bb[1].min(y);
                bb[2] = bb[2].max(x);
                bb[3] = bb[3].max(y);
            }
        }
        for i in 0..2 {
            bounds[i] = bounds[i].min(bb[i]);
            bounds[i + 2] = bounds[i + 2].max(bb[i + 2]);
        }
        min_lsb = min_lsb.min(bb[0]);
        min_rsb = min_rsb.min(aw as i16 - bb[2]);
        max_extent = max_extent.max(bb[2]);
        u16b(&mut hmtx, aw);
        i16b(&mut hmtx, bb[0]);
        max_points = max_points.max(points.len());
        max_contours = max_contours.max(contours.len());
        i16b(&mut glyf, contours.len() as i16);
        for v in bb {
            i16b(&mut glyf, v);
        }
        if !points.is_empty() {
            let mut end = 0;
            for c in &contours {
                end += c.len();
                u16b(&mut glyf, (end - 1) as u16);
            }
            u16b(&mut glyf, 0); // no instructions
                                // Mark overlapping contours so rasterizers combine bridges and cells.
            glyf.push(0x41); // ON_CURVE_POINT | OVERLAP_SIMPLE
            glyf.extend(std::iter::repeat_n(1u8, points.len() - 1));
            let mut last = 0;
            for &(x, _) in &points {
                i16b(&mut glyf, x - last);
                last = x;
            }
            last = 0;
            for &(_, y) in &points {
                i16b(&mut glyf, y - last);
                last = y;
            }
        }
        while glyf.len() % 4 != 0 {
            glyf.push(0);
        }
    }
    u32b(&mut loca, glyf.len() as u32);
    let count = keys.len() as u16 + 1;
    let mut head = vec![0; 54];
    set32(&mut head, 0, 0x00010000);
    set32(&mut head, 4, 0x00010000);
    set32(&mut head, 12, 0x5F0F3CF5);
    set16(&mut head, 16, 3);
    set16(&mut head, 18, 1000);
    // Seconds since 1904-01-01; fixed creation date makes exports deterministic.
    head[20..28].copy_from_slice(&3_787_776_000u64.to_be_bytes());
    head[28..36].copy_from_slice(&3_787_776_000u64.to_be_bytes());
    for (i, v) in bounds.iter().enumerate() {
        set16(&mut head, 36 + i * 2, *v as u16);
    }
    set16(&mut head, 46, 8);
    set16(&mut head, 48, 2);
    set16(&mut head, 50, 1);
    let mut hhea = vec![0; 36];
    set32(&mut hhea, 0, 0x00010000);
    set16(&mut hhea, 4, 850);
    set16(&mut hhea, 6, (-150i16) as u16);
    set16(&mut hhea, 8, 100);
    set16(&mut hhea, 10, advance.max(500));
    set16(&mut hhea, 12, min_lsb as u16);
    set16(&mut hhea, 14, min_rsb as u16);
    set16(&mut hhea, 16, max_extent as u16);
    set16(&mut hhea, 18, 1);
    set16(&mut hhea, 34, count);
    let mut maxp = vec![0; 32];
    set32(&mut maxp, 0, 0x00010000);
    set16(&mut maxp, 4, count);
    set16(&mut maxp, 6, max_points as u16);
    set16(&mut maxp, 8, max_contours as u16);
    set16(&mut maxp, 14, 1);
    // cmap format 4, one segment per supported ASCII code point plus sentinel.
    let n = keys.len() + 1;
    let power = 1usize << (usize::BITS - 1 - n.leading_zeros());
    let mut sub = Vec::new();
    u16b(&mut sub, 4);
    u16b(&mut sub, (16 + n * 8) as u16);
    u16b(&mut sub, 0);
    u16b(&mut sub, (n * 2) as u16);
    u16b(&mut sub, (power * 2) as u16);
    u16b(&mut sub, power.trailing_zeros() as u16);
    u16b(&mut sub, ((n - power) * 2) as u16);
    for &c in &keys {
        u16b(&mut sub, c as u16);
    }
    u16b(&mut sub, 0xffff);
    u16b(&mut sub, 0);
    for &c in &keys {
        u16b(&mut sub, c as u16);
    }
    u16b(&mut sub, 0xffff);
    for (i, &c) in keys.iter().enumerate() {
        u16b(&mut sub, (i as u16 + 1).wrapping_sub(c as u16));
    }
    u16b(&mut sub, 1);
    for _ in 0..n {
        u16b(&mut sub, 0);
    }
    let mut cmap = Vec::new();
    u16b(&mut cmap, 0);
    u16b(&mut cmap, 1);
    u16b(&mut cmap, 3);
    u16b(&mut cmap, 1);
    u32b(&mut cmap, 12);
    cmap.extend(sub);
    let family = d.family.trim();
    let ps: String = family
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .collect();
    let ps = if ps.is_empty() {
        "ParametricFont".to_string()
    } else {
        ps
    };
    let names = [
        (1, family.to_string()),
        (2, "Regular".into()),
        (3, format!("{ps};1.0")),
        (4, format!("{family} Regular")),
        (5, "Version 1.000".into()),
        (6, format!("{ps}-Regular")),
    ];
    let mut name = Vec::new();
    u16b(&mut name, 0);
    u16b(&mut name, names.len() as u16);
    u16b(&mut name, (6 + names.len() * 12) as u16);
    let mut strings = Vec::new();
    for (id, value) in names {
        let encoded: Vec<u8> = value.encode_utf16().flat_map(u16::to_be_bytes).collect();
        for v in [3, 1, 0x409, id, encoded.len() as u16, strings.len() as u16] {
            u16b(&mut name, v);
        }
        strings.extend(encoded);
    }
    name.extend(strings);
    let mut os2 = vec![0; 96];
    set16(&mut os2, 0, 4);
    set16(&mut os2, 2, advance);
    set16(&mut os2, 4, 400);
    set16(&mut os2, 6, 5);
    for (at, v) in [
        (10, 650),
        (12, 600),
        (16, 75),
        (18, 650),
        (20, 600),
        (24, 350),
        (26, 50),
        (28, 300),
    ] {
        set16(&mut os2, at, v);
    }
    set32(&mut os2, 42, 1);
    os2[58..62].copy_from_slice(b"PFE ");
    set16(&mut os2, 62, 0xC0);
    set16(&mut os2, 64, keys.first().copied().unwrap_or(' ') as u16);
    set16(&mut os2, 66, keys.last().copied().unwrap_or(' ') as u16);
    set16(&mut os2, 68, 850);
    set16(&mut os2, 70, (-150i16) as u16);
    set16(&mut os2, 72, 100);
    set16(&mut os2, 74, 850);
    set16(&mut os2, 76, 150);
    set32(&mut os2, 78, 1);
    set16(&mut os2, 86, 640);
    set16(&mut os2, 88, 800);
    set16(&mut os2, 92, 32);
    set16(&mut os2, 94, 1);
    let mut post = vec![0; 32];
    set32(&mut post, 0, 0x00030000);
    set16(&mut post, 8, (-75i16) as u16);
    set16(&mut post, 10, 50);
    for (tag, data) in [
        (*b"head", head),
        (*b"hhea", hhea),
        (*b"maxp", maxp),
        (*b"cmap", cmap),
        (*b"name", name),
        (*b"OS/2", os2),
        (*b"post", post),
        (*b"glyf", glyf),
        (*b"loca", loca),
        (*b"hmtx", hmtx),
    ] {
        tables.insert(tag, data);
    }
    let mut out = Vec::new();
    u32b(&mut out, 0x00010000);
    u16b(&mut out, 10);
    u16b(&mut out, 128);
    u16b(&mut out, 3);
    u16b(&mut out, 32);
    let mut offset = 12 + tables.len() * 16;
    let mut head_offset = 0;
    for (tag, data) in &tables {
        out.extend(tag);
        u32b(&mut out, checksum(data));
        u32b(&mut out, offset as u32);
        u32b(&mut out, data.len() as u32);
        if tag == b"head" {
            head_offset = offset;
        }
        offset += (data.len() + 3) & !3;
    }
    for data in tables.values() {
        out.extend(data);
        while out.len() % 4 != 0 {
            out.push(0);
        }
    }
    let adjustment = 0xB1B0AFBAu32.wrapping_sub(checksum(&out));
    set32(&mut out, head_offset + 8, adjustment);
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exported_font_parses_and_has_correct_metrics() {
        for (gap, round, slant) in [(0.0, 0.0, 0.0), (20.0, 25.0, -0.3), (2.0, 8.0, 0.3)] {
            let d = Design {
                gap,
                connect_cells: true,
                corner_radius: round,
                slant,
                ..Design::default()
            };
            let bytes = build(&d).unwrap();
            assert_eq!(checksum(&bytes), 0xB1B0AFBA);
            let f = ttf_parser::Face::parse(&bytes, 0).unwrap();
            assert_eq!(f.units_per_em(), 1000);
            assert_eq!(f.number_of_glyphs() as usize, d.alphabet.len() + 1);
            for (&c, cells) in &d.alphabet {
                let id = f.glyph_index(c).unwrap();
                assert!(f.glyph_hor_advance(id).unwrap() > 0);
                if !cells.is_empty() {
                    let bb = f.glyph_bounding_box(id).unwrap();
                    assert!(bb.y_min >= 0 && bb.y_max <= 800);
                    assert!(bb.x_max < f.glyph_hor_advance(id).unwrap() as i16);
                }
            }
            assert!(f.glyph_index('\u{2603}').is_none());
        }
    }
    #[test]
    fn export_is_deterministic() {
        let d = Design::default();
        assert_eq!(build(&d).unwrap(), build(&d).unwrap());
    }
}

#[cfg(all(test, target_os = "windows"))]
mod windows_tests {
    use super::*;
    #[link(name = "gdi32")]
    extern "system" {
        fn AddFontMemResourceEx(
            data: *const u8,
            size: u32,
            reserved: *mut std::ffi::c_void,
            count: *mut u32,
        ) -> *mut std::ffi::c_void;
        fn RemoveFontMemResourceEx(handle: *mut std::ffi::c_void) -> i32;
    }
    #[test]
    fn windows_accepts_generated_font() {
        for slant in [-0.3, 0.0, 0.3] {
            let d = Design {
                slant,
                connect_cells: true,
                gap: 20.0,
                corner_radius: 25.0,
                ..Design::default()
            };
            let bytes = build(&d).unwrap();
            let mut count = 0;
            // Process-private registration: no installation or persistent system changes.
            unsafe {
                let handle = AddFontMemResourceEx(
                    bytes.as_ptr(),
                    bytes.len() as u32,
                    std::ptr::null_mut(),
                    &mut count,
                );
                assert!(!handle.is_null(), "Windows rejected the exported font");
                assert_eq!(count, 1);
                assert_ne!(RemoveFontMemResourceEx(handle), 0);
            }
        }
    }
}
