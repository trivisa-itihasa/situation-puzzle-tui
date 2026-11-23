//! Core library for situation-puzzle-tui.

pub mod logging;

/// Starts the application.
///
/// # Errors
///
/// Returns an error when startup fails. The placeholder implementation
/// currently always succeeds.
pub fn run() -> std::io::Result<()> {
    tracing::info!("starting situation-puzzle-tui");
    Ok(())
}
