# Crabfull: Streaming Log Viewer

Crabfull is a simple desktop application built with Rust, using the `eframe` and `egui` libraries, for real-time log file viewing. It continuously streams and displays the contents of a log file (`log.txt`) in a scrollable, code-friendly UI.

## Features
- **Live Log Streaming:** Automatically updates the UI as new lines are appended to `log.txt`.
- **Modern GUI:** Built with `egui` for a clean, responsive interface.
- **Threaded File Watching:** Uses a background thread to monitor the log file for changes.

## Usage
1. **Build and Run**
   ```bash
   cargo run --release
   ```
   This will launch the Crabfull log viewer window.

2. **Log File**
   - The application reads from `log.txt` in the project root.
   - If `log.txt` does not exist, it will be created automatically.
   - Any new lines appended to `log.txt` will appear in the viewer in near real-time.

3. **Customizing**
   - To view a different log file, modify the `log_path` variable in `src/main.rs`.

## Dependencies
- [eframe](https://crates.io/crates/eframe)
- [egui](https://crates.io/crates/egui)
- [notify](https://crates.io/crates/notify)

## Project Structure
- `src/main.rs` — Main application logic and UI.
- `log.txt` — The log file being monitored.
- `Cargo.toml` — Project manifest and dependencies.

## License
MIT or Apache-2.0 (see [Rust licensing](https://www.rust-lang.org/policies/licenses)).

---

*Made with Rust and egui for fast, native log monitoring.*
