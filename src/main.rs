use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use eframe::egui;
use rfd::FileDialog;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "SetIntersectUnionDiff",
        options,
        Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
    )
}

struct MyApp {
    file1_path: String,
    file2_path: String,
    operation: String,
    result: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            file1_path: String::new(),
            file2_path: String::new(),
            operation: String::new(),
            result: String::new(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Set Operations");

            if ui.button("select file 1").clicked() {
                if let Some(path) = FileDialog::new().pick_file() {
                    self.file1_path = path.display().to_string();
                }
            }
            ui.label(format!("file 1: {}", self.file1_path));

            if ui.button("select file 2").clicked() {
                if let Some(path) = FileDialog::new().pick_file() {
                    self.file2_path = path.display().to_string();
                }
            }
            ui.label(format!("file 2: {}", self.file2_path));

            egui::ComboBox::from_label("select...")
                .selected_text(&self.operation)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.operation, "Intersection".to_string(), "Intersection");
                    ui.selectable_value(&mut self.operation, "Union".to_string(), "Union");
                    ui.selectable_value(&mut self.operation, "Difference".to_string(), "Difference");
                });

            if ui.button("calc").clicked() {
                self.calculate();
            }

            ui.add_space(20.0);

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add(egui::TextEdit::multiline(&mut self.result).desired_width(f32::INFINITY));
            });

            if ui.button("copy").clicked() {
                ui.output_mut(|o| o.copied_text = self.result.clone());
            }
        });
    }
}

impl MyApp {
    fn calculate(&mut self) {
        if self.file1_path.is_empty() || self.file2_path.is_empty() || self.operation.is_empty() {
            self.result = "Please select two files and an operation".to_string();
            return;
        }

        let set1 = match read_file_lines(&self.file1_path) {
            Ok(set) => set,
            Err(e) => {
                self.result = format!("read file1 error: {}", e);
                return;
            }
        };

        let set2 = match read_file_lines(&self.file2_path) {
            Ok(set) => set,
            Err(e) => {
                self.result = format!("read file2 error: {}", e);
                return;
            }
        };

        let result_set = match self.operation.as_str() {
            "Intersection" => &set1 & &set2,
            "Union" => &set1 | &set2,
            "Difference" => &set1 - &set2,
            _ => {
                self.result = "Invalid operation".to_string();
                return;
            }
        };

        self.result = result_set.into_iter().collect::<Vec<_>>().join("\n");
    }

    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

fn read_file_lines<P>(filename: P) -> io::Result<HashSet<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let mut set = HashSet::new();
    for line in reader.lines() {
        set.insert(line?);
    }
    Ok(set)
}