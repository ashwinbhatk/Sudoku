use slint::{ModelRc, VecModel};
use std::rc::Rc;

use crate::solver::solver;

slint::include_modules!();

pub fn run() -> Result<(), slint::PlatformError> {
    let window = SudokuWindow::new()?;

    // Initial puzzle (0 represents empty cells)
    let initial_grid: [[u8; 9]; 9] = [
        [5, 3, 0, 0, 7, 0, 0, 0, 0],
        [6, 0, 0, 1, 9, 5, 0, 0, 0],
        [0, 9, 8, 0, 0, 0, 0, 6, 0],
        [8, 0, 0, 0, 6, 0, 0, 0, 3],
        [4, 0, 0, 8, 0, 3, 0, 0, 1],
        [7, 0, 0, 0, 2, 0, 0, 0, 6],
        [0, 6, 0, 0, 0, 0, 2, 8, 0],
        [0, 0, 0, 4, 1, 9, 0, 0, 5],
        [0, 0, 0, 0, 8, 0, 0, 7, 9],
    ];

    // Store which cells are given (non-zero initially)
    let given_cells: Vec<bool> = initial_grid
        .iter()
        .flatten()
        .map(|&value| value != 0)
        .collect();

    let cells: Vec<i32> = initial_grid
        .iter()
        .flatten()
        .map(|&value| value as i32)
        .collect();

    let cells_model = Rc::new(VecModel::from(cells));
    let given_model = Rc::new(VecModel::from(given_cells.clone()));
    let initial_grid_ref = Rc::new(initial_grid);

    window.set_cells(ModelRc::from(cells_model.clone()));
    window.set_given_cells(ModelRc::from(given_model.clone()));

    // Handle cell value changes
    let cells_model_weak = Rc::downgrade(&cells_model);
    window.on_cell_value_changed(move |index, new_value| {
        if let Some(model) = cells_model_weak.upgrade() {
            model.set_row_data(index as usize, new_value);
        }
    });

    // Handle solve button
    let cells_model_weak = Rc::downgrade(&cells_model);
    let window_weak = window.as_weak();
    window.on_solve_clicked(move || {
        if let (Some(model), Some(window)) = (cells_model_weak.upgrade(), window_weak.upgrade()) {
            // Convert current cells to grid
            let mut grid: [[u8; 9]; 9] = [[0; 9]; 9];
            for i in 0..81 {
                let row = i / 9;
                let col = i % 9;
                grid[row][col] = model.row_data(i).unwrap_or(0) as u8;
            }

            // Solve the puzzle
            if solver(&mut grid) {
                // Update the model with solved values
                for i in 0..81 {
                    let row = i / 9;
                    let col = i % 9;
                    model.set_row_data(i, grid[row][col] as i32);
                }
                window.set_status_message("Puzzle solved successfully!".into());
                window.set_status_is_error(false);
            } else {
                window.set_status_message("No solution exists for this puzzle.".into());
                window.set_status_is_error(true);
            }
        }
    });

    // Handle clear button (clear only non-given cells)
    let cells_model_weak = Rc::downgrade(&cells_model);
    let window_weak = window.as_weak();
    let given_cells_clone = given_cells.clone();
    window.on_clear_clicked(move || {
        if let (Some(model), Some(window)) = (cells_model_weak.upgrade(), window_weak.upgrade()) {
            for i in 0..81 {
                if !given_cells_clone[i] {
                    model.set_row_data(i, 0);
                }
            }
            window.set_status_message("User entries cleared.".into());
            window.set_status_is_error(false);
        }
    });

    // Handle reset button (reset to initial state)
    let cells_model_weak = Rc::downgrade(&cells_model);
    let window_weak = window.as_weak();
    let initial_grid_clone = initial_grid_ref.clone();
    window.on_reset_clicked(move || {
        if let (Some(model), Some(window)) = (cells_model_weak.upgrade(), window_weak.upgrade()) {
            for i in 0..81 {
                let row = i / 9;
                let col = i % 9;
                model.set_row_data(i, initial_grid_clone[row][col] as i32);
            }
            window.set_status_message("Puzzle reset to original state.".into());
            window.set_status_is_error(false);
        }
    });

    window.run()
}
