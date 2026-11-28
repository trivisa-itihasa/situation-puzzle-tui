//! Core library for situation-puzzle-tui.

use crossterm::event::{self, Event, KeyCode, KeyEventKind};

pub mod logging;
pub mod ui;

/// Starts the application.
///
/// # Errors
///
/// Returns an error if the terminal cannot be initialized, or if drawing a
/// frame or reading a terminal event fails.
pub fn run() -> std::io::Result<()> {
    tracing::info!("starting situation-puzzle-tui");
    let mut terminal = ratatui::try_init()?;
    let result = run_loop(&mut terminal);
    ratatui::restore();
    result
}

fn run_loop(terminal: &mut ratatui::DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(ui::render)?;
        if let Event::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
            && matches!(key.code, KeyCode::Char('q') | KeyCode::Esc)
        {
            tracing::info!("quitting");
            break;
        }
    }
    Ok(())
}
