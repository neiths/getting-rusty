# Hecto — Terminal Text Editor in Rust

A lightweight, terminal-based text editor implemented from scratch in Rust. Built on top of [`crossterm`](https://crates.io/crates/crossterm), **Hecto** features raw terminal control, 2D scrolling, and comprehensive support for Unicode grapheme clusters and display widths.

---

## Features

- **Terminal Abstraction & Raw Mode**
  - Powered by `crossterm` with alternate screen buffer support.
  - Custom panic hook setup to ensure terminal state (raw mode, cursor visibility, screen buffer) is cleanly restored even if a panic occurs.
  - Dynamic terminal window resize handling.

- **Advanced Unicode & Grapheme Cluster Handling**
  - Uses [`unicode-segmentation`](https://crates.io/crates/unicode-segmentation) to handle multi-byte characters, combining characters, and complex grapheme clusters accurately.
  - Uses [`unicode-width`](https://crates.io/crates/unicode-width) to distinguish half-width (1 column) vs. full-width (2 columns, e.g., CJK characters and emojis).
  - Replaces zero-width characters with visual markers (`·`) so they remain perceptible.
  - Graceful boundary clipping with ellipsis (`⋯`) when graphemes span viewport edges.

- **2D Viewport & Scrolling**
  - Full vertical and horizontal scrolling tracking the text cursor.
  - Centered welcome screen banner when no file is loaded.
  - Vim-style tildes (`~`) denoting empty lines past buffer end.

- **Intuitive Cursor Navigation**
  - Arrow key navigation with automatic line wrapping at line boundaries.
  - Page jumps (`PageUp` / `PageDown`) and line jumping (`Home` / `End`).
  - Intelligent cursor snapping (`snap_to_valid_grapheme`, `snap_to_valid_line`) preventing out-of-bounds positions when navigating lines of varying lengths.

- **Text Editing**
  - Real-time character insertion, backspace, and deletion with dynamic grapheme re-clustering.
  - Automatic cursor progression, line wrapping, and viewport scrolling upon editing.

- **Safe & Idiomatic Rust**
  - Built against the Rust 2024 edition.
  - Strict Clippy lint enforcement (`pedantic`, `arithmetic_side_effects`, `as_conversions`, `integer_division`).
  - Defensively uses saturating arithmetic to eliminate underflow/overflow panic vectors.

---

## Project Structure

```
hecto/
├── Cargo.toml
├── src/
│   ├── main.rs                   # Entry point and compiler/clippy lint configuration
│   ├── editor.rs                 # Main editor loop, event evaluation, and drop hooks
│   └── editor/
│       ├── terminal.rs           # Terminal wrapper (raw mode, screen clearing, cursor control)
│       ├── editorcommand.rs      # Event-to-Command mapping (Movement, Resize, Quit)
│       ├── view.rs               # Viewport rendering, 2D scrolling, and cursor translation
│       └── view/
│           ├── buffer.rs         # In-memory document buffer and file loading
│           └── line.rs           # Grapheme segmentation, width calculation, and text fragments
```

### Module Responsibilities

| Component | Description |
| :--- | :--- |
| **`Editor`** ([`src/editor.rs`](src/editor.rs)) | Manages the editor lifecycle, event loop, panic hook, and screen refresh cycle. |
| **`Terminal`** ([`src/editor/terminal.rs`](src/editor/terminal.rs)) | Low-level terminal interface encapsulating crossterm commands, sizing, and cursor movements. |
| **`EditorCommand`** ([`src/editor/editorcommand.rs`](src/editor/editorcommand.rs)) | Parses crossterm `Event`s into domain commands (`Move`, `Resize`, `Quit`). |
| **`View`** ([`src/editor/view.rs`](src/editor/view.rs)) | Manages viewport state, scroll offsets, caret positioning, and rendering visible lines. |
| **`Buffer`** ([`src/editor/view/buffer.rs`](src/editor/view/buffer.rs)) | Holds collection of `Line`s read from disk. |
| **`Line`** ([`src/editor/view/line.rs`](src/editor/view/line.rs)) | Stores grapheme fragments, computes rendered display width, and handles horizontal clipping. |

---

## Keybindings

| Key / Shortcut | Action |
| :--- | :--- |
| <kbd>Ctrl</kbd> + <kbd>Q</kbd> | Quit editor |
| <kbd>↑</kbd> / <kbd>↓</kbd> / <kbd>←</kbd> / <kbd>→</kbd> | Move cursor up, down, left, right |
| <kbd>Home</kbd> | Move cursor to beginning of current line |
| <kbd>End</kbd> | Move cursor to end of current line |
| <kbd>Page Up</kbd> | Scroll and move cursor up by one screen page |
| <kbd>Page Down</kbd> | Scroll and move cursor down by one screen page |
| <kbd>Backspace</kbd> | Delete character to the left of the cursor |
| <kbd>Delete</kbd> | Delete character under cursor |

---

## Getting Started

### Prerequisites

- [Rust & Cargo](https://www.rust-lang.org/tools/install) (2024 edition supported / latest stable)

### Build and Run

1. **Clone the repository and navigate to the project directory:**
   ```bash
   cd hecto
   ```

2. **Launch with an empty buffer:**
   ```bash
   cargo run
   ```

3. **Open a file:**
   ```bash
   cargo run -- src/editor/test_graphemes-1.txt
   ```
   Or open any existing text file:
   ```bash
   cargo run -- <path-to-file>
   ```

---

## Roadmap

- [x] Terminal raw mode & alternate screen buffer
- [x] Panic recovery hook
- [x] Multi-directional cursor movement & boundary wrapping
- [x] 2D horizontal & vertical scrolling
- [x] Full Unicode & grapheme cluster display width calculation
- [x] Basic text editing: character insertion, backspace, and delete
- [ ] Text editing: line splitting and joining (Enter)
- [ ] Saving files & dirty buffer tracking
- [ ] Status bar & interactive command/message prompt
- [ ] Search & text matching
- [ ] Syntax highlighting
