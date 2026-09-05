use sudoku::grid;
use sudoku::solver;
fn main() {
    // let grid_default = grid::Sudoku::default(); // Initialize the Sudoku board with 0
    // grid_default.print_grid();

    let test_grid: [[u8; 9]; 9] = [
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

    let mut test_grid = grid::Sudoku::new(test_grid);
    test_grid.print_grid();
    if solver::solver(&mut test_grid.grid) {
        test_grid.print_grid();
    } else {
        println!("Solution was not found!");
    }
}
