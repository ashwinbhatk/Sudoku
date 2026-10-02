# Sudoku Solver

A fast and interactive Sudoku solver built with Rust and Slint UI framework.

## Features

- **Interactive UI**: Click cells and type numbers (1-9) to input puzzles
- **Backtracking Solver**: Efficient algorithm using bitmask optimization
- **Visual Distinction**: Given numbers shown in bold blue, user inputs in regular gray
- **Three Action Buttons**:
  - **Solve**: Solves the current puzzle state
  - **Clear**: Clears only user-entered numbers, preserving the initial puzzle
  - **Reset**: Resets the grid to the original puzzle state

## Building and Running

### Prerequisites

- Rust (1.70 or later)
- Cargo

### Build

```bash
cargo build --release
```

### Run

```bash
cargo run --release
```

## How It Works

### Solver Algorithm

The solver uses a **backtracking algorithm** with **bitmask optimization**:

1. **Bitmask tracking**: Uses three sets of bitmasks to track which numbers (1-9) are already used in:
   - Each row (9 bitmasks)
   - Each column (9 bitmasks)
   - Each 3×3 sub-grid (9 bitmasks)

2. **Backtracking**: Recursively tries valid numbers for empty cells, backtracking when no valid number can be placed

3. **Performance**: The bitmask approach provides O(1) checking for valid numbers, making the solver very fast

### Project Structure

```
src/
├── main.rs         # Entry point
├── lib.rs          # Library exports
├── grid.rs         # Sudoku grid data structure
├── bitmask.rs      # Bitmask optimization for solver
├── solver.rs       # Backtracking algorithm
├── ui_bridge.rs    # Rust-Slint bridge with callbacks
└── ui/
    └── sudoku.slint # Slint UI definition
```

## UI Architecture

- **SudokuCell**: Custom component representing individual cells with:
  - Editable TextInput for non-given cells
  - Visual styling (borders, selection, given vs. editable)
  - Input validation (only accepts 1-9 or empty)
  
- **SudokuWindow**: Main window with:
  - 9×9 grid layout with proper 3×3 sub-grid borders
  - Three action buttons (Solve, Clear, Reset)
  - Cell selection and keyboard input
  - Reactive model updates

## Usage

1. Launch the application - it loads with a sample puzzle
2. Click any editable cell (shown in regular font)
3. Type a number (1-9) to fill the cell, or press backspace/delete to clear
4. Click **Solve** to let the algorithm complete the puzzle
5. Click **Clear** to remove your entries and try solving manually
6. Click **Reset** to restore the original puzzle

## Testing

Run the test suite:

```bash
cargo test
```

Tests cover:
- Bitmask operations
- Solver correctness
- Edge cases (empty grids, invalid puzzles)

## License

MIT License - feel free to use and modify as needed.
