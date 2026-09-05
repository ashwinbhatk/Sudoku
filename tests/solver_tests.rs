use sudoku::solver::solver;

#[test]
fn solves_valid_sudoku() {
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

    let expected: [[u8; 9]; 9] = [
        [5, 3, 4, 6, 7, 8, 9, 1, 2],
        [6, 7, 2, 1, 9, 5, 3, 4, 8],
        [1, 9, 8, 3, 4, 2, 5, 6, 7],
        [8, 5, 9, 7, 6, 1, 4, 2, 3],
        [4, 2, 6, 8, 5, 3, 7, 9, 1],
        [7, 1, 3, 9, 2, 4, 8, 5, 6],
        [9, 6, 1, 5, 3, 7, 2, 8, 4],
        [2, 8, 7, 4, 1, 9, 6, 3, 5],
        [3, 4, 5, 2, 8, 6, 1, 7, 9],
    ];

    assert!(solver(&mut grid));
    assert_eq!(grid, expected);
}

#[test]
fn accepts_solved_sudoku() {
    let mut grid: [[u8; 9]; 9] = [
        [5, 3, 4, 6, 7, 8, 9, 1, 2],
        [6, 7, 2, 1, 9, 5, 3, 4, 8],
        [1, 9, 8, 3, 4, 2, 5, 6, 7],
        [8, 5, 9, 7, 6, 1, 4, 2, 3],
        [4, 2, 6, 8, 5, 3, 7, 9, 1],
        [7, 1, 3, 9, 2, 4, 8, 5, 6],
        [9, 6, 1, 5, 3, 7, 2, 8, 4],
        [2, 8, 7, 4, 1, 9, 6, 3, 5],
        [3, 4, 5, 2, 8, 6, 1, 7, 9],
    ];

    let original = grid;

    assert!(solver(&mut grid));
    assert_eq!(grid, original);
}

#[test]
fn solves_empty_sudoku() {
    let mut grid = [[0; 9]; 9];

    assert!(solver(&mut grid));

    // Check every row contains digits 1-9 exactly once.
    for row in 0..9 {
        let mut mask = 0u16;

        for col in 0..9 {
            let value = grid[row][col];

            assert!((1..=9).contains(&value));

            let bit = 1u16 << (value - 1);

            assert_eq!(mask & bit, 0);
            mask |= bit;
        }

        assert_eq!(mask, 0x01FF);
    }

    // Check every column contains digits 1-9 exactly once.
    for col in 0..9 {
        let mut mask = 0u16;

        for row in 0..9 {
            let value = grid[row][col];

            let bit = 1u16 << (value - 1);

            assert_eq!(mask & bit, 0);
            mask |= bit;
        }

        assert_eq!(mask, 0x01FF);
    }

    // Check every 3x3 sub-grid contains digits 1-9 exactly once.
    for box_row in 0..3 {
        for box_col in 0..3 {
            let mut mask = 0u16;

            for row in 0..3 {
                for col in 0..3 {
                    let value = grid[box_row * 3 + row][box_col * 3 + col];

                    let bit = 1u16 << (value - 1);

                    assert_eq!(mask & bit, 0);
                    mask |= bit;
                }
            }

            assert_eq!(mask, 0x01FF);
        }
    }
}

#[test]
fn preserves_original_values() {
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

    let original = grid;

    assert!(solver(&mut grid));

    for row in 0..9 {
        for col in 0..9 {
            if original[row][col] != 0 {
                assert_eq!(grid[row][col], original[row][col]);
            }
        }
    }
}
