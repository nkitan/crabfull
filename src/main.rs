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
        egui::CentralPanel::default().show(ctx, |ui| {
            let log = self.log_lines.lock().unwrap();
            let text = log.join("\n");
            ui.label("Log Output:");
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add(egui::TextEdit::multiline(&mut text.clone()).desired_rows(30).code_editor());
            });
        });
    }
}

fn main() {
    let log_lines = Arc::new(Mutex::new(Vec::new()));
    let log_lines_clone = log_lines.clone();
    let log_path = "log.txt".to_string();

    thread::spawn(move || {
        let mut file = File::open(&log_path).unwrap_or_else(|_| File::create(&log_path).unwrap());
        let mut pos = 0;
        loop {
            file.seek(SeekFrom::Start(pos)).ok();
            let mut reader = BufReader::new(file.try_clone().unwrap());
            let mut new_lines = Vec::new();
            for line in reader.lines() {
                if let Ok(l) = line {
                    new_lines.push(l);
                }
            }
            if !new_lines.is_empty() {
                let mut log = log_lines_clone.lock().unwrap();
                log.extend(new_lines);
            }
            pos = file.metadata().map(|m| m.len()).unwrap_or(pos);
            thread::sleep(Duration::from_millis(500));
        }
    });

    let app = LogViewerApp { log_lines };
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Streaming Log Viewer",
        native_options,
        Box::new(|_cc| Box::new(app)),
    );
}
