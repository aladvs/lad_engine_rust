#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui::*;
use tmf::TMFMesh;

//TODO :: MAKE SURE SCENES ARE PASSED ON FROM MAIN, SO EVERYTHING ACTUALLY WORKS
struct Scene {
    objects: Vec<Mesh>,
}

#[derive(Debug)] 
struct Mesh {
    vertices: Vec<(f32, f32, f32)>,
    indices: Vec<u32>,
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
        let mut input = include_bytes!("sphere.obj");
        let mut input_slice: &[u8] = input;
        let (mesh, name) = TMFMesh::read_from_obj_one(&mut input_slice).expect("Could not read TMF file!");
        let obj_vertices = mesh.get_vertices().expect("No vertices!");
      //  println!("{:#?}", obj_vertices);
        let vertices: Vec<(f32, f32, f32)> = obj_vertices
    .iter()
    .map(|&v| (v.0, v.1, v.2))
    .collect();
        let vertex_triangles = mesh.get_vertex_triangles().expect("No vertiex triangle array!");
        //let indices: Vec<u32> = vertex_triangles.to_vec();

        let dummy_mesh = Mesh {
            vertices: vertices.to_vec(),  // dummy vertex
            indices: [0, 0].to_vec(),  // dummy index
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
        let mut deltaTime = ctx.input(|ctx| ctx.predicted_dt);
      //  println!("{}",deltaTime);
        let stroke = Stroke::new(2.0, Color32::WHITE);
        egui::CentralPanel::default().show(ctx, |ui| {
            //println!("Number of objects in scene: {}", self.current_scene.objects.len()); 
            render_scene(&self.current_scene, stroke, &ui);
        });
        ctx.request_repaint();
    }
}



fn render_scene(scene: &Scene, stroke: Stroke, ui: &Ui) {
    let canvas_width = ui.ctx().screen_rect().width();
    let canvas_height = ui.ctx().screen_rect().height();
    let half_width = canvas_width / 2.0;
    let half_height = canvas_height / 2.0;


    let mut painter = ui.painter();

    for mesh in &scene.objects {
        let vertices = &mesh.vertices;

        for (i, start) in vertices.iter().enumerate() {
            for end in &vertices[i + 1..] {
                let line_start = Pos2::new(
                    start.0*100.0 + half_width,
                    start.1*100.0 + half_height,
                );
                let line_end = Pos2::new(
                    end.0*100.0 + half_width,
                    end.1*100.0 + half_height,
                );

                // Use the stroke method to draw the line segment
                painter.line_segment([line_start, line_end], stroke);
            }
        }
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