# parametric_font_editor

A lightweight, GPU-accelerated desktop tool built in Rust for designing parametric grid fonts and exporting production-ready vector assets.

![Rust](https://img.shields.io/badge/Language-Rust_2021-orange.svg)
![eframe](https://img.shields.io/badge/GUI-eframe%2Fegui_0.27-blue.svg)
![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey.svg)

---

## How It Works

### 15-Box Matrix System

Instead of drawing traditional freehand vector splines, fonts in this studio are constructed on a flexible **3 × 5 parametric grid** (15 active coordinate boxes). Every character is mapped as a unique combination of these boxes. Uppercase letters, lowercase letters, digits and common punctuation have independently editable designs.

## Geometry Engine

- **2D Vertex Nodes:** Drag interior anchors to adjust row heights and column widths. Geometry changes update the whole typeface in real time. Individual sliders offer precise control.
- **Proportional Scaling & Baseline Alignment:** Width, height and aspect ratio controls keep character dimensions consistent across the font family.
- **Shared Outlines:** Previews, PNGs, SVG artwork and TrueType fonts use the same shapes, including roundness, slant and cell connectors.

## Features

- **Input Canvas:** Select a character and click cells to draw or erase. Toggle grid guides for a cleaner canvas.
- **Live Alphabet & Text Preview:** Inspect the character gallery or type sample text at different preview sizes. Unsupported characters appear as red boxes.
- **Aspect Ratio Controls:** Precision width/height controls, ratio locking, square and tall presets, and individual row/column sliders.
- **Global Styling:** Adjust cell gaps, corner roundness, slant and letter spacing.
- **Connected Cells:** Keep neighboring active cells joined across gaps with adjustable connector thickness, including rounded cells.
- **Style Presets:** Pixel, Soft, Stencil and Linked change styling while preserving character drawings and dimensions.
- **Mirror Drawing:** Draw or erase the matching cell across the vertical center automatically.
- **Glyph Tools:** Copy/paste, clear, reset, flip or shift characters. Shifts wrap around grid edges without losing cells.
- **Design Comparison:** Capture a session-only snapshot, compare the selected glyph with its current design, and restore the captured design.
- **Undo/Redo:** Reverse edits, including grouped drag gestures and restored comparisons.
- **Editable Projects:** Save and open JSON files containing geometry, spacing and character designs.
- **TrueType Export:** Generate installable `.ttf` font files for use in system applications.
- **PNG Export:** Save antialiased black glyphs on transparent backgrounds at a 512-pixel design height.
- **SVG Export:** Generate a vector specimen sheet.

---

## Getting Started

With a Rust toolchain and Windows C++ build tools installed, run:

```sh
cargo run
```

Select a character in the gallery, edit its cells, and adjust the global controls. Both panels scroll, and the canvas scales to fit large designs.

## Save and Use Your Font

1. Enter a font family name.
2. Choose **Save project** to preserve editable source as JSON. Use **Open project** to continue editing later.
3. Choose **Export installable font (.ttf)**.
4. On Windows, open the exported TTF and choose **Install**, then select the family name in your application. Restart the application if its font list has not refreshed. Use distinct family names for variants.

Exports preserve the cell connector settings. PNG filenames use Unicode code points (for example `U+0041.png`) to avoid case collisions and invalid punctuation filenames on Windows. SVG files contain artwork; use TTF for font installation.

This is a grid-based display font editor. The three-column grid limits fine detail and small-size legibility. It does not import existing TTFs or edit Bezier outlines. TTFs are unhinted, use uniform character advances without kerning, and have a 1000-unit em with an 800-unit design height. Slant is part of the exported Regular design. Exporting does not automatically install the font.

---

## Tech Stack

- **Language:** Rust (2021 Edition)
- **GUI Framework:** `eframe` / `egui` (v0.27.2)
- **Native Dialogs:** `rfd`
- **Image Processing:** `image`
- **Project Serialization:** `serde` / `serde_json`

## Validation

```sh
cargo test
cargo clippy --all-targets -- -D warnings
```

Eight tests cover project compatibility and validation, gap-aware sizing, mirrored drawing and shifting, connected geometry, deterministic font generation, TrueType parsing, metrics and checksums. On Windows, generated fonts are loaded through process-private GDI registration and immediately removed without installing them.

---
