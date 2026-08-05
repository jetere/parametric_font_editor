# parametric_font_editor
A lightweight, GPU-accelerated desktop tool built in Rust for designing parametric grid fonts and exporting production-ready vector assets.

![Rust](https://img.shields.io/badge/Language-Rust_2021-orange.svg)
![eframe](https://img.shields.io/badge/GUI-eframe%2Fegui_0.27-blue.svg)
![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey.svg)

---

#How It Works 
## 15-Box Matrix System
Instead of drawing traditional freehand vector splines, fonts in this studio are constructed on a flexible **$3 \times 5$ parametric grid** (15 active coordinate boxes). Every character from $A\text{--}Z$ is mapped as a unique combination of these boxes.

## Preview <img width="1145" height="711" alt="Screenshot 2026-08-06 053438" src="https://github.com/user-attachments/assets/9d6242d8-d54f-4704-8fc2-2a4747114f7f" />



## Geometry Engine
* **2D Vertex Nodes & Outer Handles:** Real-time drag controls let you adjust internal row heights and column widths. Modifying a single node instantly recalculates the geometry for all 26 letters across the typeface.
* **Proportional Scaling & Baseline Locking:** Every letter is constrained to fixed boundaries, ensuring consistent cap-heights and baseline alignment across the whole font family.

## Features
- **Input Canvas:** Adjust row and column proportions with interior vertex anchors and outer edge handles.
- **$A\text{--}Z$ Alphabet Mapping:** Real-time side-by-side rendering all 26 uppercase characters.
- **Aspect Ratio Controls:** Precision width/height sliders, 1:1 aspect ratio locking, and quick presets.
- **PNG Export:** Save PNG previews for every character.
- **SVG Font Export:** Generate vector font files.

---

## Tech Stack

- **Language:** Rust (2021 Edition)
- **GUI Framework:** `eframe` / `egui` (v0.27.2)
- **Native Dialogs:** `rfd`
- **Image Processing:** `image`

---

## Inspiration & Concept

Inspired by [Niklas Tsalkos](https://www.instagram.com/niklastsalkos/)
