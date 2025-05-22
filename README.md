# Crabfull: Streaming Log Viewer

Crabfull is a powerful desktop application built with Rust, using the `eframe` and `egui` libraries, for real-time log file viewing. It continuously streams and displays the contents of a log file (`log.txt`) in a scrollable, code-friendly UI with advanced search capabilities.

## Features

### Core Features
- **Live Log Streaming:** Automatically updates the UI as new lines are appended to `log.txt`
- **Modern GUI:** Built with `egui` for a clean, responsive interface
- **Threaded File Watching:** Uses a background thread to monitor the log file for changes

### Search Capabilities
- **Real-time Search:**
  - Instant text search with match highlighting
  - Regular expression support with easy toggle
  - Smart partial-text highlighting (only matched portions are highlighted)
  - Red dot indicators for lines containing matches
  - Match counter showing total matches found
- **Search Navigation:**
  - Quick navigation between matches with ⬆⬇ buttons
  - Auto-scroll to matches when navigating
  - Maintains search state while scrolling

### Navigation Controls
- **Line Navigation:**
  - Jump to specific line number with Enter key support
  - Quick jump buttons for ±100, ±1K, ±10K lines
  - One-click bottom jump with auto-scroll disable
  - Smooth mouse wheel scrolling
  - Auto-scroll to latest content (can be toggled)

### Performance Features
- **Efficient Memory Usage:**
  - Circular buffer for memory-efficient log viewing
  - Windowed line display for handling large files
  - Smart partial file loading
  - Optimized search highlighting

### UI Features
- **Clean Interface:**
  - Monospace formatting for better readability
  - Clear line numbers with separator
  - Responsive layout adapting to window size
  - Consistent spacing and alignment

### Control Features
- **File Reading:**
  - Pause/Resume log file reading
  - Visual pause state indicator
  - Background file monitoring
- **Display:**
  - Line numbers display with separator
  - Match indicators
  - Status information display
  - Monospace formatting for better readability

## Usage
1. **Build and Run**
   ```bash
   cargo run --release
   ```
   This will launch the Crabfull log viewer window.

2. **Log File**
   - The application reads from `log.txt` in the project root
   - If `log.txt` does not exist, it will be created automatically
   - Any new lines appended to `log.txt` will appear in the viewer in near real-time

3. **Basic Controls**
   - Use the search box to find text in logs
   - Toggle "Regex" for pattern matching
   - Navigate between matches with ⬆⬇ buttons
   - Use jump controls to move through the log file
   - Enter a line number to jump to specific location
   - Click "Pause Reading" to temporarily stop updating

4. **Customizing**
   - To view a different log file, modify the `log_path` variable in `src/main.rs`

## Dependencies
- [eframe](https://crates.io/crates/eframe) - egui framework
- [egui](https://crates.io/crates/egui) - Immediate mode GUI
- [notify](https://crates.io/crates/notify) - File system notifications
- [regex](https://crates.io/crates/regex) - Regular expression support

## Project Structure
- `src/main.rs` — Main application logic and UI
- `log.txt` — The log file being monitored
- `Cargo.toml` — Project manifest and dependencies

## License
MIT or Apache-2.0 (see [Rust licensing](https://www.rust-lang.org/policies/licenses))

---

*Made with Rust and egui for fast, native log monitoring*
