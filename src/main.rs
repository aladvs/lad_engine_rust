#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui::*;

struct Scene {
    objects: Vec<Object>,
}
struct Object {
    mesh: Mesh,
    position: [f32; 3],
    rotation: [f32; 3], 
    scale: [f32; 3],
}

struct Mesh {
    vertices: Vec<u32>,
    indices: Vec<u32>,
}



fn main() -> Result<(), eframe::Error> {
   // env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "lad engine",
        options,
        Box::new(|_cc| Box::<Content>::default()),
    )
}

#[derive(Default)]
struct Content {
    text: String,
}

impl eframe::App for Content {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let stroke = Stroke::new(2.0, Color32::WHITE);
        egui::CentralPanel::default().show(ctx, |ui| {
            let line_start = Pos2::new(50.0, 50.0);
            let line_end = Pos2::new(800.0, 800.0);
            
            ui.painter().line_segment([line_start, line_end], stroke);
        });
    }
}