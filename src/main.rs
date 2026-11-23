fn main() {
    let _log_guard = match situation_puzzle_tui::logging::init() {
        Ok(guard) => guard,
        Err(error) => {
            eprintln!("failed to initialize logging: {error}");
            std::process::exit(1);
        }
    };

    if let Err(error) = situation_puzzle_tui::run() {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}
