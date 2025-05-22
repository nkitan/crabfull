use eframe::{egui, App, Frame};
use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct LogViewerApp {
    log_lines: Arc<Mutex<Vec<String>>>,
}

impl App for LogViewerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        // Request continuous repainting for real-time updates
        ctx.request_repaint();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Log Output:");
            let available_size = ui.available_size();
            egui::ScrollArea::vertical()
                .stick_to_bottom(true)
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    let log = self.log_lines.lock().unwrap();
                    let text = log.join("\n");
                    ui.add_sized(
                        available_size,
                        egui::TextEdit::multiline(&mut text.clone())
                            .desired_rows(30)
                            .code_editor()
                            .cursor_at_end(true)
                    );
                });
        });
    }
}

fn main() -> eframe::Result<()> {
    let log_lines = Arc::new(Mutex::new(Vec::new()));
    let log_lines_clone = log_lines.clone();
    let log_path = "log.txt".to_string();

    thread::spawn(move || {
        let file = File::open(&log_path).unwrap_or_else(|_| File::create(&log_path).unwrap());
        let mut reader = BufReader::new(file);
        let mut line = String::new();
        
        loop {
            match reader.read_line(&mut line) {
                Ok(0) => {
                    // EOF reached, sleep briefly and try again
                    thread::sleep(Duration::from_millis(10));
                    continue;
                }
                Ok(_) => {
                    if !line.is_empty() {
                        let mut log = log_lines_clone.lock().unwrap();
                        log.push(line.trim().to_string());
                        line.clear();
                    }
                }
                Err(_) => {
                    thread::sleep(Duration::from_millis(10));
                }
            }
        }
    });

    let native_options = eframe::NativeOptions {
        vsync: true,
        ..Default::default()
    };
    
    let app = LogViewerApp { log_lines };
    eframe::run_native(
        "Streaming Log Viewer",
        native_options,
        Box::new(|_cc| Box::new(app)),
    )
}
