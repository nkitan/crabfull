use eframe::{egui, App, Frame};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::collections::VecDeque;

/// A circular buffer that stores a fixed number of log lines
struct CircularLogBuffer {
    buffer: VecDeque<String>,
    capacity: usize,
    total_lines: usize,  // Keep track of total lines seen
}

impl CircularLogBuffer {
    fn new(capacity: usize) -> Self {
        Self {
            buffer: VecDeque::with_capacity(capacity),
            capacity,
            total_lines: 0,
        }
    }

    fn push(&mut self, line: String) {
        if self.buffer.len() >= self.capacity {
            self.buffer.pop_front();
        }
        self.buffer.push_back(line);
        self.total_lines += 1;
    }

    fn total_lines(&self) -> usize {
        self.total_lines
    }

    fn get_window(&self, start: usize, end: usize) -> Vec<(usize, &String)> {
        let buffer_start = self.total_lines.saturating_sub(self.buffer.len());
        let visible_start = start.saturating_sub(buffer_start);
        let visible_count = end.saturating_sub(start);
        
        self.buffer
            .iter()
            .skip(visible_start)
            .take(visible_count)
            .enumerate()
            .map(|(i, line)| (start + i, line))
            .collect()
    }
}

/// The main application state
struct LogViewerApp {
    log_buffer: Arc<Mutex<CircularLogBuffer>>,
    scroll_offset: f32,
    visible_lines_count: usize,
}

impl App for LogViewerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        ctx.request_repaint();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Log Output");
            let available_size = ui.available_size();
            
            // Calculate visible lines based on UI height and text size
            let text_height = ui.text_style_height(&egui::TextStyle::Monospace);
            self.visible_lines_count = (available_size.y / text_height) as usize;

            egui::ScrollArea::vertical()
                .max_height(f32::INFINITY)
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    let buffer = self.log_buffer.lock().unwrap();
                    let total_lines = buffer.total_lines();
                    
                    // Calculate visible range based on scroll position
                    let max_scroll = total_lines.saturating_sub(self.visible_lines_count);
                    let current_pos = (self.scroll_offset * max_scroll as f32) as usize;
                    
                    // Calculate window of lines to display
                    let start_line = current_pos;
                    let end_line = total_lines;  // Show all lines from current position
                    let visible_lines = buffer.get_window(start_line, end_line);
                    
                    // Create a layout to show all lines
                    for (line_num, line) in visible_lines {
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new(format!("{:>6} │ ", line_num + 1)).monospace());
                            ui.label(egui::RichText::new(line).monospace());
                        });
                    }

                    // Handle scrolling
                    let scroll_delta = ui.input(|i| i.raw_scroll_delta.y);
                    if scroll_delta != 0.0 {
                        let line_delta = scroll_delta / text_height;
                        let normalized_delta = line_delta / max_scroll.max(1) as f32;
                        self.scroll_offset = (self.scroll_offset - normalized_delta).clamp(0.0, 1.0);
                    }
                });
        });
    }
}

fn main() -> eframe::Result<()> {
    // Configure buffer size (adjust as needed)
    const BUFFER_CAPACITY: usize = 1000000; // Keep last 1,000,000 bytes in memory
    
    let log_buffer = Arc::new(Mutex::new(CircularLogBuffer::new(BUFFER_CAPACITY)));
    let log_buffer_clone = log_buffer.clone();
    let log_path = "log.txt".to_string();

    thread::spawn(move || {
        let file = File::open(&log_path).unwrap_or_else(|_| File::create(&log_path).unwrap());
        let mut reader = BufReader::new(file);
        let mut line = String::new();
        
        loop {
            match reader.read_line(&mut line) {
                Ok(0) => {
                    thread::sleep(Duration::from_millis(10));
                    continue;
                }
                Ok(_) => {
                    if !line.is_empty() {
                        let mut buffer = log_buffer_clone.lock().unwrap();
                        buffer.push(line.trim().to_string());
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
    
    let app = LogViewerApp {
        log_buffer,
        scroll_offset: 1.0, // Start at bottom
        visible_lines_count: 0,
    };

    eframe::run_native(
        "Streaming Log Viewer",
        native_options,
        Box::new(|_cc| Box::new(app)),
    )
}