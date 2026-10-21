fn main() {
    #[cfg(feature = "slint-ui")]
    slint_build::compile("src/ui/sudoku.slint").unwrap();
}
