use sudoku::bitmask::Bitmask;

#[test]
fn empty_grid_has_empty_bitmasks() {
    let grid = [[0u8; 9]; 9];

    let bitmask = Bitmask::new(&grid);

    assert_eq!(bitmask.rows, [0; 9]);
    assert_eq!(bitmask.cols, [0; 9]);
    assert_eq!(bitmask.sub_grids, [0; 9]);
}

#[test]
fn sets_correct_bit_for_value() {
    let mut grid = [[0u8; 9]; 9];

    grid[0][0] = 1;
    grid[0][1] = 5;
    grid[4][4] = 9;

    let bitmask = Bitmask::new(&grid);

    assert_eq!(bitmask.rows[0], (1 << 0) | (1 << 4));
    assert_eq!(bitmask.cols[0], 1 << 0);
    assert_eq!(bitmask.cols[1], 1 << 4);
    assert_eq!(bitmask.rows[4], 1 << 8);
}

#[test]
fn puts_value_in_correct_sub_grid() {
    let mut grid = [[0u8; 9]; 9];

    grid[0][0] = 1;
    grid[2][2] = 2;
    grid[3][3] = 3;
    grid[8][8] = 9;

    let bitmask = Bitmask::new(&grid);

    assert_eq!(bitmask.sub_grids[0], (1 << 0) | (1 << 1));
    assert_eq!(bitmask.sub_grids[4], 1 << 2);
    assert_eq!(bitmask.sub_grids[8], 1 << 8);
}
