// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{egui, epi};

pub struct SynthApp {
    volume: f32,
}

impl Default for SynthApp {
    fn default() -> Self {
        SynthApp {
            volume: 100.0,
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
            ui.add(egui::Slider::new(&mut self.volume, 0.0..=100.0)
                                .text("Volume")
                                .clamp_to_range(true)
                                .vertical());
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
