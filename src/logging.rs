//! File-based logging for the terminal UI.
//!
//! Logs go to a daily-rotated file under the platform data directory. Set
//! `RUST_LOG` to change the level, for example
//! `RUST_LOG=situation_puzzle_tui=debug`.
use std::path::PathBuf;

use directories::ProjectDirs;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::EnvFilter;

/// Errors that can occur while setting up logging.
#[derive(Debug, thiserror::Error)]
pub enum LoggingError {
    /// The platform data directory could not be determined.
    #[error("could not determine the platform data directory")]
    NoProjectDirectory,

    /// The log directory could not be created.
    #[error("failed to create log directory {path}")]
    CreateDirectory {
        /// The directory that could not be created.
        path: PathBuf,
        /// The underlying I/O error.
        #[source]
        source: std::io::Error,
    },

    /// The log file could not be initialized in the log directory.
    #[error("failed to initialize the log file in {path}")]
    InitializeAppender {
        /// The directory that contains the log file.
        path: PathBuf,
        /// The underlying appender error.
        #[source]
        source: tracing_appender::rolling::InitError,
    },

    /// A global tracing subscriber was already installed.
    #[error("a global tracing subscriber is already installed")]
    InstallSubscriber(#[from] tracing::subscriber::SetGlobalDefaultError),
}

/// Initializes file logging and returns the guard that flushes it on drop.
///
/// Keep the guard alive for as long as logging is needed. Dropping it stops
/// the background writer and flushes buffered lines.
///
/// # Errors
///
/// Returns an error if the platform data directory cannot be determined, if
/// the log directory cannot be created, if the log file cannot be initialized,
/// or if a global subscriber is already installed.
pub fn init() -> Result<WorkerGuard, LoggingError> {
    let project = ProjectDirs::from("", "", "situation-puzzle-tui")
        .ok_or(LoggingError::NoProjectDirectory)?;
    let dir = project.data_dir().join("logs");
    std::fs::create_dir_all(&dir).map_err(|source| LoggingError::CreateDirectory {
        path: dir.clone(),
        source,
    })?;

    let appender = tracing_appender::rolling::RollingFileAppender::builder()
        .rotation(tracing_appender::rolling::Rotation::DAILY)
        .filename_prefix("situation-puzzle-tui.log")
        .build(&dir)
        .map_err(|source| LoggingError::InitializeAppender {
            path: dir.clone(),
            source,
        })?;
    let (writer, guard) = tracing_appender::non_blocking(appender);

    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let subscriber = tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_ansi(false)
        .with_writer(writer)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;
    Ok(guard)
}
