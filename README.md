# Sudoku Pro & Intelligent Solver

[![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)](https://www.rust-lang.org/)
[![Slint UI](https://img.shields.io/badge/GUI-Slint%201.17-blue.svg)](https://slint.dev/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A feature-rich, high-performance Sudoku game and intelligent solver built with **Rust** and the modern **Slint UI framework**. Designed for zero-latency desktop responsiveness with subgrid bitmask constraint solving, real-time visual conflict detection, candidate pencil notes, custom puzzle input, and undo/redo history.

---

## Features

### 🎮 Gameplay & Engine
- **Puzzle Generator**: Generates valid, uniquely solvable Sudoku boards across 4 difficulty levels:
  - **Easy** (~38 clues)
  - **Medium** (~30 clues)
  - **Hard** (~26 clues)
  - **Expert** (~22 clues)
- **Fast Bitmask Solver**: $O(1)$ constraint checking using 9-bit masks (`rows`, `cols`, `sub_grids`) with recursive backtracking. Solves any valid grid in milliseconds.
- **Smart Hint Assistant**: Computes candidate masks to identify naked singles or optimal placements with contextual hint explanations.
- **Custom Setup Mode**: Input custom or newspaper puzzles manually and lock them as read-only givens.
- **Undo / Redo Stack**: Full history tracking allows reverting or repeating cell changes effortlessly.

### 🎨 Modern Graphical Interface (Slint)
- **Candidate Notes (Pencil Marks)**: 3×3 mini-grid per cell for note-taking with manual toggle and one-click **Auto Notes**.
- **Rule Conflict Highlighting**: Dynamic visual detection (crimson highlight & soft red background) for duplicate digits across rows, columns, and 3×3 boxes.
- **Crosshair & Same-Number Highlighting**: Selecting any cell illuminates identical numbers board-wide and highlights intersecting row, column, and block guides.
- **Visual Clue Distinction**: Bold blue styling for fixed puzzle clues; clear dark slate for user entries.
- **Virtual On-Screen Keypad**: 1–9 digit keypad, dedicated **Pen / Pencil Mode** toggle, and an **Erase** button.
- **Keyboard Navigation**: Full arrow-key grid navigation, digit entry (`1`–`9`), backspace/delete, pencil toggle (`P`), and undo/redo (`Ctrl+Z`, `Ctrl+Y`).
- **Game Stopwatch**: Built-in timer with pause/resume support.
- **Dark Mode / Light Mode**: Seamless dynamic theme toggle between a sleek slate dark palette and clean daylight theme.
- **Status & Feedback Banner**: Real-time notifications for hints, rule violations, resets, and victory celebrations.

---

## Architecture Overview

```
c:\Dev\Projects\Sudoku\
├── .cargo/
│   └── config.toml         # MinGW/GNU target static runtime link flags
├── src/
│   ├── main.rs             # Application entry point (GUI window / headless CLI)
│   ├── lib.rs              # Library root & feature flag gating
│   ├── bitmask.rs          # 9-bit bitmask constraint representation
│   ├── solver.rs           # Backtracking solver, conflict detector & hint system
│   ├── generator.rs        # Random board generator with unique solution verification
│   ├── history.rs          # Action stack for Undo/Redo operations
│   ├── ui_bridge.rs        # Rust <-> Slint reactive bridge & event dispatcher
│   └── ui/
│       └── sudoku.slint    # Slint declarative UI definitions & components
├── tests/
│   ├── bitmask_tests.rs    # Bitwise manipulation & subgrid mapping unit tests
│   ├── solver_tests.rs     # Solver correctness, conflicts & hints tests
│   ├── generator_tests.rs  # Puzzle generation uniqueness & clue count tests
│   └── history_tests.rs    # Undo/redo stack state transition tests
├── build.rs                # Slint build compilation script
└── Cargo.toml              # Dependencies & release optimization profile
```

---

## Keyboard Controls & Shortcuts

| Key / Shortcut | Action |
| :--- | :--- |
| `↑` `↓` `←` `→` | Navigate grid selection |
| `1` – `9` | Enter digit into selected cell (or toggle note in Pencil mode) |
| `Backspace` / `Delete` | Erase number / clear notes in selected cell |
| `P` | Toggle between Pen mode (digits) and Pencil mode (notes) |
| `Ctrl + Z` | Undo last move |
| `Ctrl + Y` | Redo last move |

---

## Getting Started

### Prerequisites
- [Rust 1.70+](https://www.rust-lang.org/tools/install) (Cargo included)
- Windows / Linux / macOS desktop environment

### 1. Build and Run GUI (Release Mode)
Run the desktop graphical application with full compiler optimizations:
```bash
cargo run --release
```

### 2. Run in Headless / CLI Mode
Run the core engine without the Slint GUI dependencies:
```bash
cargo run --release --no-default-features
```

### 3. Run the Test Suite
Execute the full suite of unit tests covering the solver, bitmasks, generator, and history:
```bash
cargo test --no-default-features
```

---

## Performance Optimizations

- **Fine-Grained Model Updates**: The UI bridge conditionally updates cell highlights and conflicts only when values change, cutting redundant Slint reactive notifications from 162 per click down to active diffs.
- **Single-Pass Grid Reading**: Grid state is cached during entry/erase events, avoiding repeated property-model reads across validation, conflict detection, and victory checks.
- **Optimized Release Profile**: Configured with `opt-level = 3`, `lto = "thin"`, and `codegen-units = 1` for minimal binary size and maximum runtime speed.

---

## License

This project is open-source under the [MIT License](LICENSE).
