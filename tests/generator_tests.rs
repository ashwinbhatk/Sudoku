use sudoku::generator::{generate_puzzle, Difficulty};
use sudoku::solver::{count_solutions, is_valid_puzzle, solver};

#[test]
fn test_generate_easy_puzzle() {
    let puzzle = generate_puzzle(Difficulty::Easy);
    assert!(is_valid_puzzle(&puzzle));

    // Must be uniquely solvable
    let solutions = count_solutions(&puzzle, 2);
    assert_eq!(solutions, 1);

    // Can be fully solved
    let mut solved = puzzle;
    assert!(solver(&mut solved));
}

#[test]
fn test_generate_medium_puzzle() {
    let puzzle = generate_puzzle(Difficulty::Medium);
    assert!(is_valid_puzzle(&puzzle));

    let solutions = count_solutions(&puzzle, 2);
    assert_eq!(solutions, 1);

    let mut solved = puzzle;
    assert!(solver(&mut solved));
}
