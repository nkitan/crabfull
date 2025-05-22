use eframe::{egui, App, Frame};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::collections::VecDeque;
use regex::Regex;

#[derive(Default)]
struct SearchState {
    query: String,
    regex: Option<Regex>,
    matches: Vec<usize>,  // Line numbers of matches
    current_match: Option<usize>,
}

impl SearchState {
    fn update_search(&mut self, new_query: String, use_regex: bool) {
        self.query = new_query;
        self.regex = if self.query.is_empty() {
            None
        } else if use_regex {
            Regex::new(&self.query).ok()
        } else {
            Regex::new(&regex::escape(&self.query)).ok()
        };
        self.matches.clear();
        self.current_match = None;
    }
    
    fn matches_pattern(&self, text: &str) -> bool {
        self.regex.as_ref().map_or(false, |re| re.is_match(text))
    }
}

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
    search: SearchState,
    use_regex: bool,
    autoscroll: bool,
}

impl App for LogViewerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        ctx.request_repaint();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Log Output");
            
            // Add search controls and options
            ui.horizontal(|ui| {
                let search_response = ui.text_edit_singleline(&mut self.search.query);
                let regex_response = ui.toggle_value(&mut self.use_regex, "Regex");
                ui.toggle_value(&mut self.autoscroll, "Auto-scroll");
                
                // Update search when query changes or regex mode toggles
                if search_response.changed() || regex_response.changed() {
                    self.search.update_search(self.search.query.clone(), self.use_regex);
                }
                
                if !self.search.query.is_empty() {
                    // Keep a reference to current matches for navigation
                    let mut matches = self.search.matches.clone();
                    matches.sort_unstable();
                    
                    if ui.button("▲").clicked() {
                        if let Some(current) = self.search.current_match {
                            if let Some(&prev) = matches.iter().rev().find(|&&i| i < current) {
                                self.search.current_match = Some(prev);
                                self.scroll_offset = prev as f32 / self.log_buffer.lock().unwrap().total_lines() as f32;
                            }
                        } else if !matches.is_empty() {
                            // If no current match, go to last match
                            let last = *matches.last().unwrap();
                            self.search.current_match = Some(last);
                            self.scroll_offset = last as f32 / self.log_buffer.lock().unwrap().total_lines() as f32;
                        }
                    }
                    
                    if ui.button("▼").clicked() {
                        if let Some(current) = self.search.current_match {
                            if let Some(&next) = matches.iter().find(|&&i| i > current) {
                                self.search.current_match = Some(next);
                                self.scroll_offset = next as f32 / self.log_buffer.lock().unwrap().total_lines() as f32;
                            }
                        } else if !matches.is_empty() {
                            // If no current match, go to first match
                            let first = *matches.first().unwrap();
                            self.search.current_match = Some(first);
                            self.scroll_offset = first as f32 / self.log_buffer.lock().unwrap().total_lines() as f32;
                        }
                    }
                    ui.label(format!("{} matches", matches.len()));
                }
            });
            
            let available_size = ui.available_size();
            
            // Calculate visible lines based on UI height and text size
            let text_height = ui.text_style_height(&egui::TextStyle::Monospace);
            self.visible_lines_count = (available_size.y / text_height) as usize;

            let scroll_area = egui::ScrollArea::vertical()
                .max_height(f32::INFINITY)
                .auto_shrink([false; 2]);

            scroll_area.show(ui, |ui| {
                    let buffer = self.log_buffer.lock().unwrap();
                    let total_lines = buffer.total_lines();
                    
                    // Calculate visible range based on scroll position
                    let max_scroll = total_lines.saturating_sub(self.visible_lines_count);
                    let current_pos = if self.autoscroll {
                        max_scroll  // When auto-scroll is on, always show the bottom
                    } else {
                        (self.scroll_offset * max_scroll as f32) as usize
                    };
                    
                    // Calculate window of lines to display
                    let start_line = current_pos;
                    let end_line = start_line + self.visible_lines_count;
                    let visible_lines = buffer.get_window(start_line, end_line);
                    
                    // First pass: collect matches
                    if !self.search.query.is_empty() {
                        self.search.matches.clear();
                        for (line_num, line) in visible_lines.iter() {
                            if self.search.matches_pattern(line) {
                                self.search.matches.push(*line_num);
                            }
                        }
                        self.search.matches.sort_unstable();
                    }

                    // Second pass: display lines with highlighting
                    for (line_num, line) in visible_lines {
                        ui.horizontal(|ui| {
                            // Line number with monospace formatting
                            ui.label(egui::RichText::new(format!("{:>6} │ ", line_num + 1))
                                .monospace());
                            
                            // Line content with highlighting for matches
                            let text = if self.search.current_match == Some(line_num) {
                                egui::RichText::new(line)
                                    .monospace()
                                    .background_color(egui::Color32::from_rgb(100, 100, 0))
                            } else if self.search.matches.contains(&line_num) {
                                egui::RichText::new(line)
                                    .monospace()
                                    .background_color(egui::Color32::from_rgb(60, 60, 0))
                            } else {
                                egui::RichText::new(line).monospace()
                            };
                            ui.label(text);
                        });
                    }

                    // Handle scrolling
                    let scroll_delta = ui.input(|i| i.raw_scroll_delta.y);
                    if scroll_delta != 0.0 {
                        self.autoscroll = false;  // Disable auto-scroll when user scrolls
                        let line_delta = scroll_delta / text_height;
                        let normalized_delta = line_delta / max_scroll.max(1) as f32;
                        self.scroll_offset = (self.scroll_offset - normalized_delta).clamp(0.0, 1.0);
                    }

                    ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
                });

                // Add control panel at bottom
                ui.horizontal(|ui| {
                    if ui.button(if self.autoscroll { "⏸ Pause" } else { "▶ Resume" }).clicked() {
                        self.autoscroll = !self.autoscroll;
                        if self.autoscroll {
                            self.scroll_offset = 1.0;  // Jump to bottom when resuming
                        }
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
        search: SearchState::default(),
        use_regex: false,
        autoscroll: true,
    };

    eframe::run_native(
        "Streaming Log Viewer",
        native_options,
        Box::new(|_cc| Box::new(app)),
    )
}