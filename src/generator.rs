use crate::bitmask::Bitmask;
use crate::solver::count_solutions;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
    Expert,
}

impl Difficulty {
    pub fn target_clues(&self) -> usize {
        match self {
            Difficulty::Easy => 38,
            Difficulty::Medium => 30,
            Difficulty::Hard => 26,
            Difficulty::Expert => 22,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Difficulty::Easy => "Easy",
            Difficulty::Medium => "Medium",
            Difficulty::Hard => "Hard",
            Difficulty::Expert => "Expert",
        }
    }
}

pub struct SimpleRng {
    state: u64,
}

impl SimpleRng {
    pub fn new() -> Self {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(0x9e3779b97f4a7c15);
        Self { state: nanos | 1 }
    }

    pub fn with_seed(seed: u64) -> Self {
        Self { state: seed | 1 }
    }

    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }

    pub fn gen_range(&mut self, min: usize, max: usize) -> usize {
        if min >= max {
            return min;
        }
        min + (self.next_u64() as usize % (max - min))
    }

    pub fn shuffle<T>(&mut self, slice: &mut [T]) {
        for i in (1..slice.len()).rev() {
            let j = self.gen_range(0, i + 1);
            slice.swap(i, j);
        }
    }
}

/// Solves a grid using randomized number ordering to produce varied solutions.
fn randomized_backtrack(
    row: usize,
    col: usize,
    grid: &mut [[u8; 9]; 9],
    bitmask_var: &mut Bitmask,
    rng: &mut SimpleRng,
) -> bool {
    if row == 9 {
        return true;
    }
    if col == 9 {
        return randomized_backtrack(row + 1, 0, grid, bitmask_var, rng);
    }
    if grid[row][col] != 0 {
        return randomized_backtrack(row, col + 1, grid, bitmask_var, rng);
    }

    let sub_grid_index = (row / 3) * 3 + (col / 3);
    let current_box = bitmask_var.rows[row]
        | bitmask_var.cols[col]
        | bitmask_var.sub_grids[sub_grid_index];

    let mut numbers = [1u8, 2, 3, 4, 5, 6, 7, 8, 9];
    rng.shuffle(&mut numbers);

    for &num in &numbers {
        let mask = 1u16 << (num - 1);
        if mask & current_box != 0 {
            continue;
        }

        bitmask_var.rows[row] |= mask;
        bitmask_var.cols[col] |= mask;
        bitmask_var.sub_grids[sub_grid_index] |= mask;
        grid[row][col] = num;

        if randomized_backtrack(row, col + 1, grid, bitmask_var, rng) {
            return true;
        }

        bitmask_var.rows[row] ^= mask;
        bitmask_var.cols[col] ^= mask;
        bitmask_var.sub_grids[sub_grid_index] ^= mask;
        grid[row][col] = 0;
    }

    false
}

/// Fills the three diagonal 3x3 boxes which are completely independent.
fn fill_diagonal_boxes(grid: &mut [[u8; 9]; 9], rng: &mut SimpleRng) {
    for box_i in 0..3 {
        let br = box_i * 3;
        let bc = box_i * 3;
        let mut digits = [1u8, 2, 3, 4, 5, 6, 7, 8, 9];
        rng.shuffle(&mut digits);

        let mut idx = 0;
        for r in 0..3 {
            for c in 0..3 {
                grid[br + r][bc + c] = digits[idx];
                idx += 1;
            }
        }
    }
}

/// Generates a complete, valid Sudoku solution.
pub fn generate_complete_board(rng: &mut SimpleRng) -> [[u8; 9]; 9] {
    let mut grid = [[0u8; 9]; 9];
    fill_diagonal_boxes(&mut grid, rng);
    let mut bitmask_var = Bitmask::new(&grid);
    randomized_backtrack(0, 0, &mut grid, &mut bitmask_var, rng);
    grid
}

/// Generates a valid Sudoku puzzle for the given difficulty with a guaranteed unique solution.
pub fn generate_puzzle(difficulty: Difficulty) -> [[u8; 9]; 9] {
    let mut rng = SimpleRng::new();
    let mut full = generate_complete_board(&mut rng);
    let target = difficulty.target_clues();

    // Generate symmetrical pairs of positions (r, c) and (8 - r, 8 - c)
    let mut positions = Vec::new();
    for r in 0..5 {
        let max_c = if r == 4 { 5 } else { 9 };
        for c in 0..max_c {
            positions.push((r, c));
        }
    }
    rng.shuffle(&mut positions);

    let mut remaining = 81;

    for (r, c) in positions {
        if remaining <= target {
            break;
        }

        let sym_r = 8 - r;
        let sym_c = 8 - c;

        let val1 = full[r][c];
        let val2 = full[sym_r][sym_c];

        full[r][c] = 0;
        full[sym_r][sym_c] = 0;

        let count = if r == sym_r && c == sym_c { 1 } else { 2 };

        // Test if unique solution still holds
        if count_solutions(&full, 2) == 1 {
            remaining -= count;
        } else {
            // Restore
            full[r][c] = val1;
            full[sym_r][sym_c] = val2;
        }
    }

    full
}
