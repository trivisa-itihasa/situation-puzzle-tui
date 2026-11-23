//! Core library for situation-puzzle-tui

/// Starts the application.
///
/// # Errors
///
/// Returns an arror when startup failes. The placeholder implementation
/// currently always scceeds.
pub fn run() -> std::io::Result<()> {
    println!("situation-puzzle-tui");
    Ok(())
}
