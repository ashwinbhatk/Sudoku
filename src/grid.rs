pub struct Sudoku {
    pub grid: [[u8; 9]; 9],
}

// Initializes Sudoku with default = 0
impl Default for Sudoku {
    fn default() -> Self {
        Self { grid: [[0; 9]; 9] }
    }
}

impl Sudoku {
    pub fn new(test_grid: [[u8; 9]; 9]) -> Self {
        Self { grid: test_grid }
    }

    pub fn print_grid(&self) {
        for row in 0..9 {
            for col in 0..9 {
                print!("{} ", self.grid[row][col]);
            }
            println!();
        }
        println!();
    }
}
