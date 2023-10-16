#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui::*;

struct Scene {
    objects: Vec<Mesh>,
}

struct Mesh {
    vertices: Vec<f64>,
    indices: Vec<i32>,
    position: [f32; 3],
    rotation: [f32; 3], 
    scale: [f32; 3],
}

impl Default for Scene {
    fn default() -> Self {
        Scene {
            objects: Vec::new(),
        }
    }
}



fn main() -> Result<(), eframe::Error> {
   // env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions::default();
        let mut content = Content {
        text: String::new(),
        // Initialize the scene
        scene: initialize_scene(),
    };
    eframe::run_native(
        "lad engine",
        options,
        Box::new(|_cc| Box::<Content>::default()),
    )
}

#[derive(Default)]
struct Content {
    text: String,
    scene: Scene,
}

impl eframe::App for Content {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let stroke = Stroke::new(2.0, Color32::WHITE);
        egui::CentralPanel::default().show(ctx, |ui| {
            render_scene(&self.scene, stroke);
            let line_start = Pos2::new(50.0, 50.0);
            let line_end = Pos2::new(800.0, 800.0);
            
            ui.painter().line_segment([line_start, line_end], stroke);
        });
    }
}

fn render_scene(scene: &Scene, stroke: Stroke) {
    for mesh in &scene.objects {
        println!("{}", mesh.vertices[0]);  // Print the first element of vertices for each mesh
    }
}

fn initialize_scene() -> Scene {
    // Create a sample scene with some objects
    let objects = vec![
        Mesh {
            vertices: vec![/* vertices for the first object */
    // Front face
    -100.0, -100.0, 100.0,
    100.0, -100.0, 100.0,
    100.0, 100.0, 100.0,
    -100.0, 100.0, 100.0,
    // Back face
    -100.0, -100.0, -100.0,
    100.0, -100.0, -100.0,
    100.0, 100.0, -100.0,
    -100.0, 100.0, -100.0,
        ],
            indices: vec![        // Front face
            0, 1, 2, 0, 2, 3,
            // Back face
            4, 5, 6, 4, 6, 7,
            // Top face
            3, 2, 6, 3, 6, 7,
            // Bottom face
            0, 1, 5, 0, 5, 4,
            // Left face
            0, 3, 7, 0, 7, 4,
            // Right face
            1, 2, 6, 1, 6, 5,
            ],
            position: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0],
            scale: [1.0, 1.0, 1.0],
        },
        // Add more objects as needed
    ];

    Scene { objects }
}