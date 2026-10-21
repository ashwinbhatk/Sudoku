#[cfg(feature = "slint-ui")]
fn main() -> Result<(), slint::PlatformError> {
    sudoku::ui_bridge::run()
}

#[cfg(not(feature = "slint-ui"))]
fn main() {
    println!("Sudoku Pro Engine: CLI mode active. Run with --features slint-ui to launch GUI.");
}
