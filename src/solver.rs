use crate::bitmask;

pub fn backtrack(
    row_index: usize,
    col_index: usize,
    grid: &mut [[u8; 9]; 9],
    bitmask_var: &mut bitmask::Bitmask,
) -> bool {
    // End of Sudoku
    if row_index == 9 {
        return true;
    }

    // End of row, move to next
    if col_index == 9 {
        return backtrack(row_index + 1, 0, grid, bitmask_var);
    }

    // Row major iteration, move to next col
    if grid[row_index][col_index] != 0 {
        return backtrack(row_index, col_index + 1, grid, bitmask_var);
    }

    let sub_grid_index: usize = (row_index / 3) * 3 + (col_index / 3);

    // Combine all digits in row, col & sub_grid
    let current_box: u16 = bitmask_var.rows[row_index]
        | bitmask_var.cols[col_index]
        | bitmask_var.sub_grids[sub_grid_index];

    for it in 1..=9 {
        // Mask for current number
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

        // Backtracking & restoring values
        bitmask_var.rows[row_index] ^= mask;
        bitmask_var.cols[col_index] ^= mask;
        bitmask_var.sub_grids[sub_grid_index] ^= mask;
        grid[row_index][col_index] = 0;
    }

    // No num was filled
    false
}

pub fn solver(grid: &mut [[u8; 9]; 9]) -> bool {
    let mut bitmask_var = bitmask::Bitmask::new(grid);
    backtrack(0, 0, grid, &mut bitmask_var)
}
