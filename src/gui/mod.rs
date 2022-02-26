#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{egui, epi};

pub struct SynthApp {
    name: String,
    age: u32,
}

impl Default for SynthApp {
    fn default() -> Self {
        SynthApp {
            name: "Thompson".to_owned(),
            age: 24,
        }
    }
}

impl epi::App for SynthApp {
    fn name(&self) -> &str {
        "My egui App"
    }

    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Click each year").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));
        });

        // Resize the native window to be just the size we need it to be
        frame.set_window_size(ctx.used_size());
    }
}

pub fn start_gui() {
    let app = SynthApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
