#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui::*;
use std::fs::File;
use std::io::{BufReader, Cursor};
use obj::{load_obj, Obj};



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

struct Content {
    text: String,
    current_scene: Scene,
    speed_slider: (f32, f32, f32),
    pos_slider: (f32, f32, f32),
}

impl Default for Content {
    fn default() -> Self {
        Content {
            text: String::new(),
            current_scene: Scene::default(),
            speed_slider: (0.0, 0.0, 0.0), // Set the default value to 50.0
            pos_slider: (0.0, 0.0, 0.0),
        }
    }
}



impl Default for Scene {
    fn default() -> Self {
        const TEAPOT_OBJ_BYTES: &'static [u8] = include_bytes!("models/suzanne.obj");
        let teapot_obj_bytes = Cursor::new(TEAPOT_OBJ_BYTES);
        let input = BufReader::new(teapot_obj_bytes);
        let dome: Obj = load_obj(input).expect("AAAA");
        
        let mut mesh_vertices = vec![];  // Create an empty vector to store mesh vertices
        let mut mesh_indices = vec![];   // Create an empty vector to store mesh indices

        for index in &dome.indices {
            // Add each vertex index to the mesh indices
            mesh_indices.push(*index as u32); // Convert to u32 if necessary
        }

        for vertex in &dome.vertices {
            // Access the 'position' field to get the vertex coordinates
            let position = vertex.position;
            let mesh_vertex = (position[0] as f32, position[1] as f32, position[2] as f32);
            mesh_vertices.push(mesh_vertex);
        }

        let dummy_mesh = Mesh {
            vertices: mesh_vertices,  // Use the converted mesh vertices
            indices: mesh_indices,   // Use the converted mesh indices
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
        let mut deltaTime = ctx.input(|ctx| ctx.stable_dt);
       // self.current_scene.objects[0].rotation[1] += 0.6 * deltaTime;
       // self.current_scene.objects[0].rotation[0] += 0.6 * deltaTime;

      //  println!("{}",deltaTime);
        let stroke = Stroke::new(0.5, Color32::WHITE);

        egui::CentralPanel::default().show(ctx, |ui| {
            render_scene(&self.current_scene, stroke, &ui);


            Frame::popup(ui.style())
            .stroke(Stroke::NONE)
            .show(ui, |ui| {
                ui.set_max_width(170.0);
                CollapsingHeader::new("Settings")
                .show(ui, |ui| settings_menu(ui, self, deltaTime))
            });
            self.current_scene.objects[0].rotation[0] += (self.speed_slider.0 * 0.1) * deltaTime;
            self.current_scene.objects[0].rotation[1] += (self.speed_slider.1 * 0.1) * deltaTime;
            self.current_scene.objects[0].rotation[2] += (self.speed_slider.2 * 0.1) * deltaTime;
            //rotation_ui(ui, self, deltaTime);

            //println!("Number of objects in scene: {}", self.current_scene.objects.len()); 
        });
        ctx.request_repaint();
    }
}



fn apply_rotation(vertex: (f32, f32, f32), angles: [f32; 3]) -> [f32; 3] {
    let sin_x = f32::sin(angles[0]);
    let cos_x = f32::cos(angles[0]);
    let sin_y = f32::sin(angles[1]);
    let cos_y = f32::cos(angles[1]);
    let sin_z = f32::sin(angles[2]);
    let cos_z = f32::cos(angles[2]);

    let mut result = [0.0, 0.0, 0.0];

    // Apply yaw (Z-axis) rotation
    result[0] = cos_z * vertex.0 - sin_z * vertex.1;
    result[1] = sin_z * vertex.0 + cos_z * vertex.1;
    result[2] = vertex.2;

    // Apply pitch (Y-axis) rotation
    let temp_x = cos_y * result[0] - sin_y * result[2];
    result[2] = sin_y * result[0] + cos_y * result[2];
    result[0] = temp_x;

    // Apply roll (X-axis) rotation
    let temp_y = cos_x * result[1] - sin_x * result[2];
    result[2] = sin_x * result[1] + cos_x * result[2];
    result[1] = temp_y;

    result
}

fn calculate_normal(vertex_a: [f32; 3], vertex_b: [f32; 3], vertex_c: [f32; 3]) -> [f32; 3] {
    let edge1 = [
        vertex_b[0] - vertex_a[0],
        vertex_b[1] - vertex_a[1],
        vertex_b[2] - vertex_a[2],
    ];
    
    let edge2 = [
        vertex_c[0] - vertex_a[0],
        vertex_c[1] - vertex_a[1],
        vertex_c[2] - vertex_a[2],
    ];
    
    // Calculate the cross product of the two edges to get the normal
    let normal = [
        edge1[1] * edge2[2] - edge1[2] * edge2[1],
        edge1[2] * edge2[0] - edge1[0] * edge2[2],
        edge1[0] * edge2[1] - edge1[1] * edge2[0],
    ];
    
    // Normalize the normal
    let length = (normal[0].powi(2) + normal[1].powi(2) + normal[2].powi(2)).sqrt();
    
    [
        normal[0] / length,
        normal[1] / length,
        normal[2] / length,
    ]
}


fn render_scene(scene: &Scene, stroke: Stroke, ui: &Ui) {
    let canvas_width = ui.ctx().screen_rect().width();
    let canvas_height = ui.ctx().screen_rect().height();
    let half_width = canvas_width / 2.0;
    let half_height = canvas_height / 2.0;

    let mut painter = ui.painter();
    
    let mut triangles_with_depth: Vec<(usize, usize, usize, f32)> = Vec::new();

    for mesh in &scene.objects {
        let vertices = &mesh.vertices;
        let indices = &mesh.indices;

        for i in (0..indices.len()).step_by(3) {
            let a = indices[i] as usize;
            let b = indices[i + 1] as usize;
            let c = indices[i + 2] as usize;

            let vertex_a = &vertices[a];
            let vertex_b = &vertices[b];
            let vertex_c = &vertices[c];

            // Apply the rotation to the vertices using the separate function
            let rotated_a = apply_rotation(*vertex_a, mesh.rotation);
            let rotated_b = apply_rotation(*vertex_b, mesh.rotation);
            let rotated_c = apply_rotation(*vertex_c, mesh.rotation);

            let posed_a = [
                rotated_a[0] + &mesh.position[0],
                rotated_a[1] + &mesh.position[1],
                rotated_a[2] + &mesh.position[2],
            ];

            let posed_b = [
                rotated_b[0] + &mesh.position[0],
                rotated_b[1] + &mesh.position[1],
                rotated_b[2] + &mesh.position[2],
            ];

            let posed_c = [
                rotated_c[0] + &mesh.position[0],
                rotated_c[1] + &mesh.position[1],
                rotated_c[2] + &mesh.position[2],
            ];

            // Backface Culling: Skip back-facing triangles
            let normal = calculate_normal(posed_a, posed_b, posed_c);
            if normal[2] < 0.0 {
                // Normal points away from the camera, so it's a front face
                continue;
            }
            

            let depth = (posed_a[2] + posed_b[2] + posed_c[2]) / 3.0;

            triangles_with_depth.push((a, b, c, depth));
        }
    }

    // Sort triangles by depth
    triangles_with_depth.sort_by(|a, b| a.3.partial_cmp(&b.3).unwrap_or(std::cmp::Ordering::Equal));

    for (a, b, c, _) in triangles_with_depth {
        let vertices = &scene.objects[0].vertices;
        let vertex_a = &vertices[a];
        let vertex_b = &vertices[b];
        let vertex_c = &vertices[c];

        // Apply the rotation to the vertices using the separate function
        let rotated_a = apply_rotation(*vertex_a, scene.objects[0].rotation);
        let rotated_b = apply_rotation(*vertex_b, scene.objects[0].rotation);
        let rotated_c = apply_rotation(*vertex_c, scene.objects[0].rotation);

        let posed_a = [
            rotated_a[0] + &scene.objects[0].position[0],
            rotated_a[1] + &scene.objects[0].position[1],
            rotated_a[2] + &scene.objects[0].position[2],
        ];

        let posed_b = [
            rotated_b[0] + &scene.objects[0].position[0],
            rotated_b[1] + &scene.objects[0].position[1],
            rotated_b[2] + &scene.objects[0].position[2],
        ];

        let posed_c = [
            rotated_c[0] + &scene.objects[0].position[0],
            rotated_c[1] + &scene.objects[0].position[1],
            rotated_c[2] + &scene.objects[0].position[2],
        ];

        // Invert the Y-axis to render upside down
        let line_start_a = [
            posed_a[0] * 100.0 + half_width,
            canvas_height - posed_a[1] * 100.0 - half_height,
        ];

        let line_end_b = [
            posed_b[0] * 100.0 + half_width,
            canvas_height - posed_b[1] * 100.0 - half_height,
        ];

        let line_end_c = [
            posed_c[0] * 100.0 + half_width,
            canvas_height - posed_c[1] * 100.0 - half_height,
        ];

        let points = vec![
            Pos2::new(line_start_a[0], line_start_a[1]),
            Pos2::new(line_end_b[0], line_end_b[1]),
            Pos2::new(line_end_c[0], line_end_c[1]),
        ];
        let stroke_black = Stroke::new(0.5, Color32::BLACK);
        let shape = Shape::convex_polygon(points, value_to_color(posed_c[2]), stroke_black);
        painter.add(shape);
    }
}



fn value_to_color (value: f32) -> Color32{
let clamped_value = value.clamp(-1.0, 1.0);
let interpolation_factor = 1.0 - (clamped_value+ 1.0) / 2.0;

let red = (255.0- interpolation_factor * 255.0) as u8;
let green = (255.0 - interpolation_factor * 255.0) as u8;
let blue = (255.0 - interpolation_factor * 255.0) as u8;

Color32::from_rgb(red, green, blue)
}

fn settings_menu(ui: &mut Ui, reference : &mut Content, deltaTime: f32) {
    
    gerneral_settings(ui, reference, deltaTime);
    
        ui.add_space(10.0);
        ui.separator();

    ui.add(TextEdit::singleline(&mut "Rotation Settings:").desired_width(110.0));
    
        ui.add_space(4.0);

    rotation_ui(ui, reference, deltaTime);

        ui.add_space(10.0);
        ui.separator();
        ui.add_space(4.0);
    
    transform_ui(ui, reference, deltaTime);
}

fn gerneral_settings(ui: &mut Ui, reference : &mut Content, deltaTime: f32) {
    ui.set_min_width(0.0);
    ui.add(TextEdit::singleline(&mut "Placeholder").desired_width(110.0));
}

fn rotation_ui(ui: &mut Ui, reference : &mut Content, deltaTime: f32) {
    ui.set_min_width(0.0);
  //  ui.horizontal(|ui| {
        ui.vertical(|ui| {
            ui.add(TextEdit::singleline(&mut "X Rotation Speed").desired_width(110.0));

            ui.add(egui::Slider::new(&mut reference.speed_slider.0, 0.0..=100.0));

            if (ui.button("Reset").clicked()) {
                reference.current_scene.objects[0].rotation[0] = 0.0;
                reference.speed_slider.0 = 0.0
            }
        });

        ui.vertical(|ui| {
            ui.add(TextEdit::singleline(&mut "Y Rotation Speed").desired_width(110.0));
            ui.add(egui::Slider::new(&mut reference.speed_slider.1, 0.0..=100.0));

            if (ui.button("Reset").clicked()) {
                reference.current_scene.objects[0].rotation[1] = 0.0;
                reference.speed_slider.1 = 0.0
            }

        });
        ui.vertical(|ui| {
            ui.add(TextEdit::singleline(&mut "Z Rotation Speed").desired_width(110.0));
            ui.add(egui::Slider::new(&mut reference.speed_slider.2, 0.0..=100.0));

            if (ui.button("Reset").clicked()) {
                reference.current_scene.objects[0].rotation[2] = 0.0;
                reference.speed_slider.2 = 0.0
            }
        });
   // });
   ui.add_space(10.0);

}

fn transform_ui(ui: &mut Ui, reference : &mut Content, deltaTime: f32) {
    ui.set_min_width(0.0);
    //  ui.horizontal(|ui| {
          ui.vertical(|ui| {
              ui.add(TextEdit::singleline(&mut "X").desired_width(110.0));
              ui.add(egui::Slider::new(&mut reference.pos_slider.0, -5.0..=5.0).clamp_to_range(false));
          });
  
          ui.vertical(|ui| {
              ui.add(TextEdit::singleline(&mut "Y").desired_width(110.0));
              ui.add(egui::Slider::new(&mut reference.pos_slider.1, -5.0..=5.0).clamp_to_range(false));
  
  
          });
          ui.vertical(|ui| {
              ui.add(TextEdit::singleline(&mut "Z").desired_width(110.0));
              ui.add(egui::Slider::new(&mut reference.pos_slider.2, -5.0..=5.0).clamp_to_range(false));
          });
          reference.current_scene.objects[0].position[0] = reference.pos_slider.0;
          reference.current_scene.objects[0].position[1] = reference.pos_slider.1;
          reference.current_scene.objects[0].position[2] = reference.pos_slider.2;
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