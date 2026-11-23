fn main() {
    if let Err(error) = situation_puzzle_tui::run() {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}
