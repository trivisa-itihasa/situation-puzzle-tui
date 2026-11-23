//! Core library for situation-puzzle-tui

/// Starts the application.
///
/// # Errors
///
/// Returns an error when startup fails. The placeholder implementation
/// currently always succeeds.
pub fn run() -> std::io::Result<()> {
    println!("situation-puzzle-tui");
    Ok(())
}
