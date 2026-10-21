use crate::bitmask::Bitmask;

/// Recursive backtracking solver using bitmasks for O(1) checks.
pub fn backtrack(
    row_index: usize,
    col_index: usize,
    grid: &mut [[u8; 9]; 9],
    bitmask_var: &mut Bitmask,
) -> bool {
    if row_index == 9 {
        return true;
    }

    if col_index == 9 {
        return backtrack(row_index + 1, 0, grid, bitmask_var);
    }

    if grid[row_index][col_index] != 0 {
        return backtrack(row_index, col_index + 1, grid, bitmask_var);
    }

    let sub_grid_index: usize = (row_index / 3) * 3 + (col_index / 3);
    let current_box: u16 = bitmask_var.rows[row_index]
        | bitmask_var.cols[col_index]
        | bitmask_var.sub_grids[sub_grid_index];

    for it in 1..=9 {
        let mask: u16 = 1 << (it - 1);

        if mask & current_box != 0 {
            continue;
        }

        bitmask_var.rows[row_index] |= mask;
        bitmask_var.cols[col_index] |= mask;
        bitmask_var.sub_grids[sub_grid_index] |= mask;
        grid[row_index][col_index] = it;

        if backtrack(row_index, col_index + 1, grid, bitmask_var) {
            return true;
        }

        bitmask_var.rows[row_index] ^= mask;
        bitmask_var.cols[col_index] ^= mask;
        bitmask_var.sub_grids[sub_grid_index] ^= mask;
        grid[row_index][col_index] = 0;
    }

    false
}

/// Solves a Sudoku grid in-place. Returns true if a solution was found.
pub fn solver(grid: &mut [[u8; 9]; 9]) -> bool {
    if !is_valid_puzzle(grid) {
        return false;
    }
    let mut bitmask_var = Bitmask::new(grid);
    backtrack(0, 0, grid, &mut bitmask_var)
}

/// Backtracking solver that counts solutions up to a given limit.
fn count_solutions_backtrack(
    row: usize,
    col: usize,
    grid: &mut [[u8; 9]; 9],
    bitmask_var: &mut Bitmask,
    count: &mut usize,
    max: usize,
) {
    if *count >= max {
        return;
    }

    if row == 9 {
        *count += 1;
        return;
    }

    if col == 9 {
        count_solutions_backtrack(row + 1, 0, grid, bitmask_var, count, max);
        return;
    }

    if grid[row][col] != 0 {
        count_solutions_backtrack(row, col + 1, grid, bitmask_var, count, max);
        return;
    }

    let sub_grid_index: usize = (row / 3) * 3 + (col / 3);
    let current_box: u16 = bitmask_var.rows[row]
        | bitmask_var.cols[col]
        | bitmask_var.sub_grids[sub_grid_index];

    for it in 1..=9 {
        let mask: u16 = 1 << (it - 1);

        if mask & current_box != 0 {
            continue;
        }

        bitmask_var.rows[row] |= mask;
        bitmask_var.cols[col] |= mask;
        bitmask_var.sub_grids[sub_grid_index] |= mask;
        grid[row][col] = it;

        count_solutions_backtrack(row, col + 1, grid, bitmask_var, count, max);

        bitmask_var.rows[row] ^= mask;
        bitmask_var.cols[col] ^= mask;
        bitmask_var.sub_grids[sub_grid_index] ^= mask;
        grid[row][col] = 0;

        if *count >= max {
            return;
        }
    }
}

/// Counts solutions for a puzzle up to a limit (e.g. 2 to check uniqueness).
pub fn count_solutions(grid: &[[u8; 9]; 9], max: usize) -> usize {
    if !is_valid_puzzle(grid) {
        return 0;
    }
    let mut copy = *grid;
    let mut bitmask_var = Bitmask::new(&copy);
    let mut count = 0;
    count_solutions_backtrack(0, 0, &mut copy, &mut bitmask_var, &mut count, max);
    count
}

/// Returns true if no duplicate non-zero digits exist in any row, column, or 3x3 subgrid.
pub fn is_valid_puzzle(grid: &[[u8; 9]; 9]) -> bool {
    // Check rows
    for r in 0..9 {
        let mut seen = 0u16;
        for c in 0..9 {
            let val = grid[r][c];
            if val != 0 {
                let mask = 1u16 << (val - 1);
                if seen & mask != 0 {
                    return false;
                }
                seen |= mask;
            }
        }
    }

    // Check columns
    for c in 0..9 {
        let mut seen = 0u16;
        for r in 0..9 {
            let val = grid[r][c];
            if val != 0 {
                let mask = 1u16 << (val - 1);
                if seen & mask != 0 {
                    return false;
                }
                seen |= mask;
            }
        }
    }

    // Check 3x3 sub-grids
    for b in 0..9 {
        let mut seen = 0u16;
        let br = (b / 3) * 3;
        let bc = (b % 3) * 3;
        for r in 0..3 {
            for c in 0..3 {
                let val = grid[br + r][bc + c];
                if val != 0 {
                    let mask = 1u16 << (val - 1);
                    if seen & mask != 0 {
                        return false;
                    }
                    seen |= mask;
                }
            }
        }
    }

    true
}

/// Finds all cells (represented as a 1D flat array of 81 booleans) that are currently in conflict.
pub fn find_conflicts(grid: &[[u8; 9]; 9]) -> [bool; 81] {
    let mut conflicts = [false; 81];

    for r in 0..9 {
        for c in 0..9 {
            let val = grid[r][c];
            if val == 0 {
                continue;
            }

            // Check row
            for c2 in 0..9 {
                if c != c2 && grid[r][c2] == val {
                    conflicts[r * 9 + c] = true;
                    conflicts[r * 9 + c2] = true;
                }
            }

            // Check column
            for r2 in 0..9 {
                if r != r2 && grid[r2][c] == val {
                    conflicts[r * 9 + c] = true;
                    conflicts[r2 * 9 + c] = true;
                }
            }

            // Check subgrid
            let br = (r / 3) * 3;
            let bc = (c / 3) * 3;
            for dr in 0..3 {
                for dc in 0..3 {
                    let r2 = br + dr;
                    let c2 = bc + dc;
                    if (r != r2 || c != c2) && grid[r2][c2] == val {
                        conflicts[r * 9 + c] = true;
                        conflicts[r2 * 9 + c2] = true;
                    }
                }
            }
        }
    }

    conflicts
}

/// Returns a bitmask (bits 0..8 corresponding to 1..9) of valid candidates for a cell.
pub fn get_candidates_bitmask(row: usize, col: usize, grid: &[[u8; 9]; 9]) -> u16 {
    if grid[row][col] != 0 {
        return 0;
    }
    let bitmask = Bitmask::new(grid);
    let sub_grid_index = (row / 3) * 3 + (col / 3);
    let used = bitmask.rows[row] | bitmask.cols[col] | bitmask.sub_grids[sub_grid_index];
    (!used) & 0x01FF
}

/// Provides a hint: returns (row, col, value, explanation).
pub fn get_hint(grid: &[[u8; 9]; 9]) -> Option<(usize, usize, u8, &'static str)> {
    if !is_valid_puzzle(grid) {
        return None;
    }

    let mut solved = *grid;
    if !solver(&mut solved) {
        return None;
    }

    // 1. Look for a Naked Single (cell with exactly 1 valid candidate in current state)
    for r in 0..9 {
        for c in 0..9 {
            if grid[r][c] == 0 {
                let mask = get_candidates_bitmask(r, c, grid);
                if mask.count_ones() == 1 {
                    let val = (mask.trailing_zeros() + 1) as u8;
                    return Some((r, c, val, "Naked Single: Only one number can legally fit in this cell!"));
                }
            }
        }
    }

    // 2. Otherwise find the first empty cell with the fewest candidates
    let mut best_cell = None;
    let mut min_candidates = 10;

    for r in 0..9 {
        for c in 0..9 {
            if grid[r][c] == 0 {
                let mask = get_candidates_bitmask(r, c, grid);
                let count = mask.count_ones();
                if count > 0 && count < min_candidates {
                    min_candidates = count;
                    best_cell = Some((r, c));
                }
            }
        }
    }

    if let Some((r, c)) = best_cell {
        return Some((r, c, solved[r][c], "Hint: Strategic placement identified for this cell."));
    }

    None
}
