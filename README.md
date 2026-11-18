# Parametric Font Studio

A native Rust/egui editor for stylized 3 x 5 pixel fonts.

## Run

With a Rust toolchain and Windows C++ build tools installed:

```sh
cargo run
```

## Edit

- Select a character in the gallery and click cells to toggle them.
- Drag orange nodes to resize rows and columns; use individual dimension sliders for precision.
- Adjust global width, height, aspect ratio, cell gap, roundness, slant and letter spacing.
- Copy/paste characters, flip horizontally or vertically, clear or reset individual glyphs.
- Undo/redo edits. Drag gestures are grouped into a single undo step.
- Type sample text and adjust preview size. Uppercase, lowercase, digits and common punctuation have independent editable designs. Unsupported characters appear as red boxes in the preview.
- Both panels scroll, and the editing canvas scales to fit large designs.

This remains a grid-based display font editor: it does not import existing TTFs or edit Bezier outlines. Three columns impose limits on the detail and legibility of complex letters, particularly at small sizes.

## Save and use fonts

1. Enter a font family name.
2. **Save project** writes JSON containing geometry, spacing and every character design. **Open project** restores it for further editing.
3. **Export installable font (.ttf)** creates a TrueType font with Unicode mappings, outlines, metrics, names and checksums.
4. On Windows, open the exported TTF and choose **Install**, then select the family name in your application. Restart the target application if its font list has not refreshed. Use distinct family names for variants.

TTFs use a 1000-unit em and an 800-unit design height. They are unhinted, with uniform character advances and no kerning. Slant is part of the exported Regular design. Exporting does not automatically install the font.

PNG previews are antialiased black glyphs on transparent backgrounds at a 512-pixel design height. Files are named by Unicode code point (for example `U+0041.png`) to avoid case collisions and invalid punctuation filenames on Windows. SVG export creates a vector specimen sheet, not an installable SVG font. All outputs share the same geometry, including rounding and slant.

## Validation

```sh
cargo test
cargo clippy --all-targets -- -D warnings
```

Tests cover project round trips and validation, gap-aware size presets, deterministic font generation, TrueType parsing, character mappings, bounding boxes and checksums. On Windows, a test loads generated fonts through GDI using process-private memory registration and removes them immediately; it does not install fonts.

## Connected cells and new editing tools

- **Connect neighboring cells** bridges horizontal and vertical active neighbors across gaps. Adjust **Connector thickness** to keep thin lines or broad strokes between rounded cells. These bridges are included in PNG, SVG, TTF and project saves. Turn the option off for separated cells.
- **Grid lines** hides the outlines on the editing canvas; these guides are never exported.
- **Style presets**: Pixel, Soft, Stencil and Linked change styling while preserving character drawings and dimensions.
- **Mirror drawing** toggles the corresponding cell across the vertical center when you draw or erase.
- **Move glyph** shifts a character left/right/up/down with wraparound, preserving all active cells.
- **Capture comparison** stores a session-only snapshot and shows the selected glyph beside its current design. Restore the entire captured design or dismiss the comparison. Restoration supports undo; save a project to preserve a snapshot between sessions.
