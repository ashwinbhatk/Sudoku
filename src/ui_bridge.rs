use slint::{ModelRc, VecModel};
use std::rc::Rc;

use crate::solver::solver;

slint::include_modules!();

pub fn run() -> Result<(), slint::PlatformError> {
    let window = SudokuWindow::new()?;

    let mut grid: [[u8; 9]; 9] = [
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

    solver(&mut grid);

    let cells: Vec<i32> = grid.iter().flatten().map(|&value| value as i32).collect();

    let model = Rc::new(VecModel::from(cells));

    window.set_cells(ModelRc::from(model));

    window.run()
}
