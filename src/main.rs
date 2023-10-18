#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui::*;
//TODO :: MAKE SURE SCENES ARE PASSED ON FROM MAIN, SO EVERYTHING ACTUALLY WORKS
struct Scene {
    objects: Vec<Mesh>,
}

#[derive(Debug)] 
struct Mesh {
    vertices: Vec<f64>,
    indices: Vec<i32>,
    position: [f32; 3],
    rotation: [f32; 3], 
    scale: [f32; 3],
}



fn main() -> Result<(), eframe::Error> {
   // env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions::default();
        let mut content = Content {
        text: String::new(),
    };
    eframe::run_native(
        "lad engine",
        options,
        Box::new(|_cc| {
            let my_scene = create_scene();
            let my_app = App::from_scene(my_scene);
            // or my_app = App { scene: my_scene, ... };
            Box::new(my_app)
          }
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
            println!("Number of objects in scene: {}", current_scene.objects.len());  // Debug print
            render_scene(current_scene, stroke);
            let line_start = Pos2::new(50.0, 50.0);
            let line_end = Pos2::new(800.0, 800.0);
            
            ui.painter().line_segment([line_start, line_end], stroke);
        });
    }
}

fn initialize_scene() -> Scene {
    println!("Initializing scene...");

    let dummy_mesh = Mesh {
        vertices: vec![0.0, 0.0, 0.0],  // A dummy vertex
        indices: vec![0],  // A dummy index
        position: [0.0, 0.0, 0.0],
        rotation: [0.0, 0.0, 0.0],
        scale: [1.0, 1.0, 1.0],
    };

    println!("Dummy mesh: {:?}", dummy_mesh);  // Debug print

    Scene {
        objects: vec![dummy_mesh],  // Adding the dummy mesh to the scene
    }
}



fn render_scene(scene: &Scene, stroke: Stroke) {
    println!("Rendering scene...");

    println!("Length of objects: {}", scene.objects.len());  // Debug print

    if scene.objects.is_empty() {
        println!("No objects in the scene.");
    } else {
        println!("Number of objects: {}", scene.objects.len());

        for (i, mesh) in scene.objects.iter().enumerate() {
            println!("Mesh {}: {:?}", i, mesh);
        }
    }
}