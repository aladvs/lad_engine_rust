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
   // let options = eframe::NativeOptions::default();
   let options = eframe::NativeOptions {
    icon_data: Some(load_icon()),
    ..Default::default()
};
        let mut content = Content {
        text: String::default(),
        current_scene: Scene::default()
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
    current_scene: Scene,
}

impl Default for Scene {
    fn default() -> Self {
        let dummy_mesh = Mesh {
            vertices: vec![0.0, 0.0, 0.0],  // dummy vertex
            indices: vec![0],  // dummy index
            position: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0],
            scale: [1.0, 1.0, 1.0],
        };
    
        println!("Dummy mesh: {:?}", dummy_mesh);  
    
        Scene {
            objects: vec![dummy_mesh], 
        }
    }
}

impl eframe::App for Content {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let stroke = Stroke::new(2.0, Color32::WHITE);
        egui::CentralPanel::default().show(ctx, |ui| {
            println!("Number of objects in scene: {}", self.current_scene.objects.len()); 
            render_scene(&self.current_scene, stroke, &ui);
        });
    }
}

fn initialize_scene() -> Scene {
    println!("Initializing scene...");

    let dummy_mesh = Mesh {
        vertices: vec![0.0, 0.0, 0.0],  // dummy vertex
        indices: vec![0],  // dummy index
        position: [0.0, 0.0, 0.0],
        rotation: [0.0, 0.0, 0.0],
        scale: [1.0, 1.0, 1.0],
    };

    println!("Dummy mesh: {:?}", dummy_mesh);  

    Scene {
        objects: vec![dummy_mesh], 
    }
}



fn render_scene(scene: &Scene, stroke: Stroke, ui: &Ui) {
    println!("Rendering scene...");

    println!("Length of objects: {}", scene.objects.len());  

        for (i, mesh) in scene.objects.iter().enumerate() {
            println!("Mesh {}: {:?}", i, mesh);
            let line_start = Pos2::new(50.0, 50.0);
            let line_end = Pos2::new(800.0, 800.0);
            
            ui.painter().line_segment([line_start, line_end], stroke);
        }
    }

    pub(crate) fn load_icon() -> eframe::IconData {
        let (icon_rgba, icon_width, icon_height) = {
            let icon = include_bytes!("LadLogo.png");
            let image = image::load_from_memory(icon)
                .expect("Failed to open icon path")
                .into_rgba8();
            let (width, height) = image.dimensions();
            let rgba = image.into_raw();
            (rgba, width, height)
        };
        
        eframe::IconData {
            rgba: icon_rgba,
            width: icon_width,
            height: icon_height,
        }
    }