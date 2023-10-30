#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui::*;
use std::fs::File;
use std::io::{BufReader, Cursor};
use obj::{load_obj, Obj};



struct Scene {
    camera_position : [f32; 3],
    camera_rotation : [f32; 3],
    objects: Vec<Mesh>,
    light: Light,
}

struct Light {
    position: [f32; 3],
    intensity: f32,
}

#[derive(Debug)] 
struct Mesh {
    name: String,
    vertices: Vec<(f32, f32, f32)>,
    indices: Vec<u32>,
    position: [f32; 3],
    rotation: [f32; 3], 
}




fn main() -> Result<(), eframe::Error> {
   let options = eframe::NativeOptions {
    icon_data: Some(load_icon()),
    drag_and_drop_support: true,
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
    selected_object: Option<usize>,
    rotation_index: Option<usize>,
    dropped_files: Vec<egui::DroppedFile>,
}

impl Default for Content {
    fn default() -> Self {
        Content {
            text: String::new(),
            current_scene: Scene::default(),
            speed_slider: (0.0, 10.0, 0.0), 
            selected_object: Some(0),
            rotation_index: Some(0),
            dropped_files: vec!(),
        }
    }
}



impl Default for Scene {
    fn default() -> Self {
        Scene {
            camera_position: [0.0, 0.0, 0.0],
            camera_rotation: [0.0, 0.0, 0.0],
            objects: vec![
                obj_to_mesh(include_bytes!("models/suzanne.obj"), [1.6, 0.7, -1.3], "Suzanne"), 
                obj_to_mesh(include_bytes!("models/mario.obj"), [0.0, 0.0, 0.0], "Mario")
                ],
            light: Light {position: [1.5, 2.2, 4.5], intensity: 16.4},
        }
    }
}

fn obj_to_mesh(bytes:&'static [u8], position: [f32; 3], name: &str) -> Mesh {
    let OBJ_BYTES: &'static [u8] = bytes;

    let obj_bytes = Cursor::new(OBJ_BYTES);
    let input = BufReader::new(obj_bytes);
    let mesh: Obj = load_obj(input).expect("AAAA");
    
    let mut mesh_vertices = vec![];  
    let mut mesh_indices = vec![];  

    for index in &mesh.indices {
        mesh_indices.push(*index as u32); 
    }

    for vertex in &mesh.vertices {
        let position = vertex.position;
        let mesh_vertex = (position[0] as f32, position[1] as f32, position[2] as f32);
        mesh_vertices.push(mesh_vertex);
    }

    let output = Mesh {
        name: name.to_string(),
        vertices: mesh_vertices,  
        indices: mesh_indices,   
        position,
        rotation: [0.0, 0.0, 0.0],
    };

    output
}

fn drag_to_mesh(bytes: &Option<std::sync::Arc<[u8]>>, position: [f32; 3], name: &str) -> Mesh {
    let mut output = Mesh {
        name: "error".to_string(),
        vertices: vec![],
        indices: vec![],
        position,
        rotation: [0.0, 0.0, 0.0],
    };

    if let Some(data) = bytes.as_ref().map(|data| data.as_ref()) {
        let obj_bytes = Cursor::new(data);
        let input = BufReader::new(obj_bytes);

        // Use the '?' operator to handle the potential error from load_obj
        let mesh: Obj = match load_obj(input) {
            Ok(mesh) => mesh,
            Err(error) => {
                eprintln!("Error loading OBJ: {:?}", error);
                return output; // Return early on error
            }
        };

        let mut mesh_vertices = vec![];
        let mut mesh_indices = vec![];

        for index in &mesh.indices {
            mesh_indices.push(*index as u32);
        }

        for vertex in &mesh.vertices {
            let position = vertex.position;
            let mesh_vertex = (position[0] as f32, position[1] as f32, position[2] as f32);
            mesh_vertices.push(mesh_vertex);
        }

        output = Mesh {
            name: name.to_string(),
            vertices: mesh_vertices,
            indices: mesh_indices,
            position,
            rotation: [0.0, 0.0, 0.0],
        };
    }

    output
}

impl eframe::App for Content {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut deltaTime = ctx.input(|ctx| ctx.stable_dt);
        let stroke = Stroke::new(0.5, Color32::WHITE);

        handle_input(&mut self.current_scene, ctx, deltaTime);

        egui::CentralPanel::default().show(ctx, |ui| {
            // -----------------------------
            // * drag and drop handling *
            if !self.dropped_files.is_empty() {
                ui.group(|ui| {
                    ui.label("Dropped files:");

                    for file in &self.dropped_files {
                        println!("{:#?}", file);
                        let mut info = if let Some(path) = &file.path {
                            path.display().to_string()
                        } else if !file.name.is_empty() {
                            file.name.clone()
                        } else {
                            "???".to_owned()
                        };

                        if Option::is_some(&file.bytes) {
                            self.current_scene.objects.append(&mut vec![drag_to_mesh(&file.bytes, [0.0,0.0,0.0], &info.as_str())]);
                        }

                        if !cfg!(target_arch = "wasm32") {
                            if let Some(path) = &file.path {
                                match File::open(&path) {
                                    Ok(file2) => {
                                        let mut input = BufReader::new(file2);
                                        // Now you can use 'input' for reading from the file.
                                        let mesh: Obj = match load_obj(input) {
                                            Ok(mesh) => mesh,
                                            Err(err) => {
                                                eprintln!("Error loading OBJ: {:?}", err);
                                                // Handle the error appropriately.
                                                return; // Exit the function or handle the error in another way
                                            }
                                        };
                        
                                        let mut mesh_vertices = vec![];
                                        let mut mesh_indices = vec![];
                        
                                        for index in &mesh.indices {
                                            mesh_indices.push(*index as u32);
                                        }
                        
                                        for vertex in &mesh.vertices {
                                            let position = vertex.position;
                                            let mesh_vertex = (position[0] as f32, position[1] as f32, position[2] as f32);
                                            mesh_vertices.push(mesh_vertex);
                                        }
                        
                                        let output = Mesh {
                                            name: "Imported Obj".to_string(),
                                            vertices: mesh_vertices,
                                            indices: mesh_indices,
                                            position: [0.0, 0.0, 0.0],
                                            rotation: [0.0, 0.0, 0.0],
                                        };
                        
                                        self.current_scene.objects.append(&mut vec![output]);
                                    }
                                    Err(err) => {
                                        eprintln!("Error opening file: {:?}", err);
                                        // Handle the error appropriately.
                                    }
                                }
                            }
                        }
                        


                        let mut additional_info = vec![];
                        if !file.mime.is_empty() {
                            additional_info.push(format!("type: {}", file.mime));
                        }
                        if let Some(bytes) = &file.bytes {
                            additional_info.push(format!("{} bytes", bytes.len()));
                        }
                        if !additional_info.is_empty() {
                            info += &format!(" ({})", additional_info.join(", "));
                        }

                        ui.label(info);
                         }
                         self.dropped_files = vec!();
                });
                    
            }

            preview_files_being_dropped(ctx);

            ctx.input(|i| {
                if !i.raw.dropped_files.is_empty() {
                    self.dropped_files = i.raw.dropped_files.clone();
                }
            });

            // * Drag and drop ^
            // * SUPER bad and doesnt handle errors but i'm pretty sure it won't crash
            // -----------------------

            render_scene(&self.current_scene, stroke, &ui);


            Frame::popup(ui.style())
            .stroke(Stroke::NONE)
            .show(ui, |ui| {
                ui.set_max_width(170.0);
                CollapsingHeader::new("Settings")
                .show(ui, |ui| settings_menu(ui, self, deltaTime))
            });

            if let Some(index) = self.rotation_index {
                if let Some(object) = self.current_scene.objects.get_mut(index) {
                    object.rotation[0] += self.speed_slider.0 * 10.0 * deltaTime;
                    object.rotation[1] += self.speed_slider.1 * 10.0 * deltaTime;
                    object.rotation[2] += self.speed_slider.2 * 10.0 * deltaTime;
            
                    /*
                    * Limit rotation amount without changing effective rotation
                    object.rotation[0] = object.rotation[0].rem_euclid(360.0);
                    object.rotation[1] = object.rotation[1].rem_euclid(360.0);
                    object.rotation[2] = object.rotation[2].rem_euclid(360.0);
                    * Commented because stuttering issues???
                     */
                }
            }
            
        });
        ctx.request_repaint();
    }
}

fn preview_files_being_dropped(ctx: &egui::Context) {
    use egui::*;
    use std::fmt::Write as _;

    if !ctx.input(|i| i.raw.hovered_files.is_empty()) {
        let text = ctx.input(|i| {
            let mut text = "Dropping files:\n".to_owned();
            for file in &i.raw.hovered_files {
                if let Some(path) = &file.path {
                    write!(text, "\n{}", path.display()).ok();
                } else if !file.mime.is_empty() {
                    write!(text, "\n{}", file.mime).ok();
                } else {
                    text += "\n???";
                }
            }
            text
        });

        let painter =
            ctx.layer_painter(LayerId::new(Order::Foreground, Id::new("file_drop_target")));

        let screen_rect = ctx.screen_rect();
        painter.rect_filled(screen_rect, 0.0, Color32::from_black_alpha(192));
        painter.text(
            screen_rect.center(),
            Align2::CENTER_CENTER,
            text,
            TextStyle::Heading.resolve(&ctx.style()),
            Color32::WHITE,
        );
    }
}

fn handle_input(reference : &mut Scene, ctx : &Context, deltaTime: f32) {
    let camera_rotation = reference.camera_rotation[1];
    let move_speed = 10.0 * deltaTime;
    //Yes, this is a mess.
    //No, I don't care.
    //Calculates direction based on camera Y rotation.

    if ctx.input(|i| i.key_down(Key::W)) {
        reference.camera_position[0] -= camera_rotation.to_radians().sin() * move_speed;
        reference.camera_position[2] += camera_rotation.to_radians().cos() * move_speed;
    }
    if ctx.input(|i| i.key_down(Key::S)) {
        reference.camera_position[0] += camera_rotation.to_radians().sin() * move_speed;
        reference.camera_position[2] -= camera_rotation.to_radians().cos() * move_speed;
    }
    if ctx.input(|i| i.key_down(Key::A)) {
        reference.camera_position[0] += (camera_rotation.to_radians() + std::f32::consts::FRAC_PI_2).sin() * move_speed;
        reference.camera_position[2] -= (camera_rotation.to_radians() + std::f32::consts::FRAC_PI_2).cos() * move_speed;
    }
    if ctx.input(|i| i.key_down(Key::D)) {
        reference.camera_position[0] -= (camera_rotation.to_radians() + std::f32::consts::FRAC_PI_2).sin() * move_speed;
        reference.camera_position[2] += (camera_rotation.to_radians() + std::f32::consts::FRAC_PI_2).cos() * move_speed;
    }
    if ctx.input(|i| i.key_down(Key::ArrowLeft)) {
        reference.camera_rotation[1] -= (7000.0 * deltaTime).to_radians();
    }
    if ctx.input(|i| i.key_down(Key::ArrowRight)) {
        reference.camera_rotation[1] += (7000.0 * deltaTime).to_radians();
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

    // X-axis
    result[0] = vertex.0;
    result[1] = cos_x * vertex.1 - sin_x * vertex.2;
    result[2] = sin_x * vertex.1 + cos_x * vertex.2;

    // Y-axis
    let temp_x = cos_y * result[0] + sin_y * result[2];
    result[2] = -sin_y * result[0] + cos_y * result[2];
    result[0] = temp_x;

    // Z-axis
    let temp_x = cos_z * result[0] - sin_z * result[1];
    result[1] = sin_z * result[0] + cos_z * result[1];
    result[0] = temp_x;

    result
}


fn render_scene(scene: &Scene, stroke: Stroke, ui: &Ui) {
    let canvas_width = ui.ctx().screen_rect().width();
    let canvas_height = ui.ctx().screen_rect().height();
    let half_width = canvas_width / 2.0;
    let half_height = canvas_height / 2.0;


    let mut mesh = egui::Mesh::default();
    let mut triangles_with_depth: Vec<(usize, [Pos2; 3], f32, Color32)> = Vec::new();

    for (object_index, mesh) in scene.objects.iter().enumerate() {
        let vertices = &mesh.vertices;
        let indices = &mesh.indices;
        let rotation = &[
            mesh.rotation[0].to_radians(),
            mesh.rotation[1].to_radians(),
            mesh.rotation[2].to_radians(),
        ];
        let position = &mesh.position;

        for i in (0..indices.len()).step_by(3) {
            let a = indices[i] as usize;
            let b = indices[i + 1] as usize;
            let c = indices[i + 2] as usize;
            let vertex_a = &vertices[a];
            let vertex_b = &vertices[b];
            let vertex_c = &vertices[c];

            let rotated_a = apply_rotation(*vertex_a, *rotation);
            let rotated_b = apply_rotation(*vertex_b, *rotation);
            let rotated_c = apply_rotation(*vertex_c, *rotation);

            let pose_a = [
                rotated_a[0] + position[0],
                rotated_a[1] + position[1],
                rotated_a[2] + position[2],
            ];

            let pose_b = [
                rotated_b[0] + position[0],
                rotated_b[1] + position[1],
                rotated_b[2] + position[2],
            ];

            let pose_c = [
                rotated_c[0] + position[0],
                rotated_c[1] + position[1],
                rotated_c[2] + position[2],
            ];

            //Lighting is calculated here, as everything after takes the camera into account. Lighting should not be camera dependent.
            let lighting_a = calculate_lighting(pose_a, pose_b, pose_c, scene.light.position, scene.light.intensity , 5000.0);

            let posed_a = [
                pose_a[0] + scene.camera_position[0],
                pose_a[1] + scene.camera_position[1],
                pose_a[2] + scene.camera_position[2] - 10.0,
            ];

            let posed_b = [
                pose_b[0] + scene.camera_position[0],
                pose_b[1] + scene.camera_position[1],
                pose_b[2] + scene.camera_position[2] - 10.0,
            ];

            let posed_c = [
                pose_c[0] + scene.camera_position[0],
                pose_c[1] + scene.camera_position[1],
                pose_c[2] + scene.camera_position[2] - 10.0,
            ];

            let mut final_a = apply_rotation(
                (posed_a[0], posed_a[1], posed_a[2]),
                [
                    scene.camera_rotation[0].to_radians(),
                    scene.camera_rotation[1].to_radians(),
                    scene.camera_rotation[2].to_radians(),
                ],
            );

            let mut final_b = apply_rotation(
                (posed_b[0], posed_b[1], posed_b[2]),
                [
                    scene.camera_rotation[0].to_radians(),
                    scene.camera_rotation[1].to_radians(),
                    scene.camera_rotation[2].to_radians(),
                ],
            );

            let mut final_c = apply_rotation(
                (posed_c[0], posed_c[1], posed_c[2]),
                [
                    scene.camera_rotation[0].to_radians(),
                    scene.camera_rotation[1].to_radians(),
                    scene.camera_rotation[2].to_radians(),
                ],
            );

            final_a = [
                final_a[0],
                final_a[1],
                final_a[2] + 10.0,
            ];
            final_b = [
                final_b[0],
                final_b[1],
                final_b[2] + 10.0,
            ];
            final_c = [
                final_c[0],
                final_c[1],
                final_c[2] + 10.0,
            ];

            let depth = -(final_a[2] + final_b[2] + final_c[2]) / 3.0;

            let depth_a = final_a[2] * -0.1;
            let depth_b = final_b[2] * -0.1;
            let depth_c = final_c[2] * -0.1;

            if depth_a > -1.0 && depth_b > -1.0 && depth_c > -1.0 {
                let perspective_factor_a = 1.0 / (1.0 + depth_a);
                let perspective_factor_b = 1.0 / (1.0 + depth_b);
                let perspective_factor_c = 1.0 / (1.0 + depth_c);

                let line_start_a = [
                    final_a[0] * perspective_factor_a * 100.0 + half_width,
                    canvas_height - final_a[1] * perspective_factor_a * 100.0 - half_height,
                ];

                let line_end_b = [
                    final_b[0] * perspective_factor_b * 100.0 + half_width,
                    canvas_height - final_b[1] * perspective_factor_b * 100.0 - half_height,
                ];

                let line_end_c = [
                    final_c[0] * perspective_factor_c * 100.0 + half_width,
                    canvas_height - final_c[1] * perspective_factor_c * 100.0 - half_height,
                ];

                let triangle = [
                    Pos2::new(line_start_a[0], line_start_a[1]),
                    Pos2::new(line_end_b[0], line_end_b[1]),
                    Pos2::new(line_end_c[0], line_end_c[1]),
                ];

                let lighting = value_to_color((lighting_a[0] + lighting_a[1] + lighting_a[2]) / 3.0, 0.0, 1.0 );
                triangles_with_depth.push((object_index, triangle, depth, lighting));
            }
        }
    }

 triangles_with_depth.sort_by(|a, b| {
    b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal).then_with(|| a.0.cmp(&b.0))
});
    for (_, triangle, _, lighting) in triangles_with_depth {
        let color = lighting;

        mesh.colored_vertex(triangle[0], color);
        mesh.colored_vertex(triangle[1], color);
        mesh.colored_vertex(triangle[2], color);

        let vertex_count = mesh.vertices.len() as u32;
        mesh.add_triangle(vertex_count - 3, vertex_count - 2, vertex_count - 1);
    }

    ui.painter().add(egui::Shape::mesh(mesh));
}

fn calculate_lighting(
    vertex_a: [f32; 3],
    vertex_b: [f32; 3],
    vertex_c: [f32; 3],
    light_position: [f32; 3],
    intensity: f32,
    max_distance: f32,
) -> [f32; 3] {
    let mut normal = calculate_normal(vertex_a, vertex_b, vertex_c);
    normal = [
        normal[0],
        normal[1],
        normal[2],
        ];

    // Calculate the vector from the triangle vertices to the light source
    let to_light = [
        light_position[0] - vertex_a[0],
        light_position[1] - vertex_a[1],
        light_position[2] - vertex_a[2],
    ];

    // Calculate the distance from the light source to the triangle
    let distance = f32::sqrt(to_light[0] * to_light[0] + to_light[1] * to_light[1] + to_light[2] * to_light[2]);

    if distance > max_distance {
        // Light is too far away, no lighting
        return [0.0, 0.0, 0.0];
    }

    // Normalize the to_light vector
    let to_light_length = f32::sqrt(to_light[0] * to_light[0] + to_light[1] * to_light[1] + to_light[2] * to_light[2]);
    let to_light_normalized = [
        to_light[0] / to_light_length,
        to_light[1] / to_light_length,
        to_light[2] / to_light_length,
    ];

    // Calculate the cosine of the angle between the normal and the to_light vector
    let cos_theta = normal[0] * to_light_normalized[0] + normal[1] * to_light_normalized[1] + normal[2] * to_light_normalized[2];

    if cos_theta <= 0.0 {
        // Light is behind the triangle, no lighting
        return [0.0, 0.0, 0.0];
    }

    //Lambert's Cosine Law
    let lighting_intensity = intensity * cos_theta / (distance * distance);

    [lighting_intensity, lighting_intensity, lighting_intensity]
}

fn calculate_normal(vertex_a: [f32; 3], vertex_b: [f32; 3], vertex_c: [f32; 3]) -> [f32; 3] {
    // Calculate the cross product of two edges of the triangle to find the normal vector
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

    let normal = [
        edge1[1] * edge2[2] - edge1[2] * edge2[1],
        edge1[2] * edge2[0] - edge1[0] * edge2[2],
        edge1[0] * edge2[1] - edge1[1] * edge2[0],
    ];

    // Normalize the normal vector
    let normal_length = f32::sqrt(normal[0] * normal[0] + normal[1] * normal[1] + normal[2] * normal[2]);
    [
        normal[0] / normal_length,
        normal[1] / normal_length,
        normal[2] / normal_length,
    ]
}





fn value_to_color(value: f32, min_value: f32, max_value: f32) -> Color32 {

    let clamped_value = value.clamp(min_value, max_value);

    // Map the clamped value to the range [0.0, 1.0]
    let interpolation_factor = (clamped_value - min_value) / (max_value - min_value);

    // Calculate the color components
    //Change the 20.0 to change the base color
    let red 
    = (20.0 + (interpolation_factor * 255.0)) as u8;
    let green
    = (20.0 + (interpolation_factor * 255.0)) as u8;
    let blue
    = (20.0 + (interpolation_factor * 255.0)) as u8;

    Color32::from_rgb(red, green, blue)
}

/*
 * --------------------------------------------
 *                    UI
*/

fn settings_menu(ui: &mut Ui, reference : &mut Content, deltaTime: f32) {
    egui::ScrollArea::vertical().show(ui, |ui| {
    scene_view(ui, reference, deltaTime);

        ui.add_space(10.0);
        ui.separator();
    
    if reference.selected_object != None {
        transform_ui(ui, reference, deltaTime);

        ui.add_space(10.0);
        ui.separator();
    }



    gerneral_settings(ui, reference, deltaTime);

        ui.add_space(10.0);
        ui.separator();
        ui.add_space(4.0);

    camera_settings(ui, reference, deltaTime);
    
    });
}

fn scene_view(ui: &mut Ui, reference : &mut Content, deltaTime: f32) {
    ui.add(TextEdit::singleline(&mut "Scene:").desired_width(110.0)); 

    for (index, mesh) in reference.current_scene.objects.iter_mut().enumerate() {
        let mut enabled = false;
        if Some(index) == reference.selected_object {
            enabled = true
        }
        ui.horizontal(|ui| {
        ui.add(TextEdit::singleline(&mut "        ").desired_width(8.0));
        if ui.toggle_value(&mut enabled, format!("{}", mesh.name)).clicked() {
            reference.selected_object = Some(index);
            enabled = true
        }
    });
    }
    ui.add(TextEdit::singleline(&mut "To import an OBJ file,").desired_width(130.0)); 
    ui.add(TextEdit::singleline(&mut "just drag and drop it").desired_width(130.0)); 
    ui.add(TextEdit::singleline(&mut "onto the window.").desired_width(130.0)); 

    ui.add(TextEdit::singleline(&mut "Only triangulated").desired_width(130.0)); 
    ui.add(TextEdit::singleline(&mut "meshes are").desired_width(130.0)); 
    ui.add(TextEdit::singleline(&mut "supported.").desired_width(130.0)); 
}

fn transform_ui(ui: &mut Ui, reference : &mut Content, deltaTime: f32) {
    ui.set_min_width(0.0);

        if let Some(selected_object) = reference.selected_object {
        ui.add(egui::TextEdit::singleline(&mut reference.current_scene.objects[selected_object].name));
        }
    
        ui.add_space(4.0);

        ui.add(TextEdit::singleline(&mut "Transform:").desired_width(110.0));
        ui.add_space(4.0);

        ui.horizontal(|ui| {
            if let Some(selected_object) = reference.selected_object {
                if selected_object < reference.current_scene.objects.len() {
                    ui.add(egui::DragValue::new(&mut reference.current_scene.objects[selected_object].position[0]).speed(0.05));  
                    ui.add(egui::DragValue::new(&mut reference.current_scene.objects[selected_object].position[1]).speed(0.05));  
                    ui.add(egui::DragValue::new(&mut reference.current_scene.objects[selected_object].position[2]).speed(0.05));
                }
            } 
            
        });
    
        ui.add_space(4.0);
        
        rotation_ui(ui, reference, deltaTime);
}

fn rotation_ui(ui: &mut Ui, reference : &mut Content, deltaTime: f32) {
    ui.vertical(|ui| {
        ui.add(TextEdit::singleline(&mut "Rotation:").desired_width(110.0));


        ui.horizontal(|ui| {
            if let Some(selected_object) = reference.selected_object {
                if selected_object < reference.current_scene.objects.len() {
                    ui.add(egui::DragValue::new(&mut reference.current_scene.objects[selected_object].rotation[0]).speed(0.1));  
                    ui.add(egui::DragValue::new(&mut reference.current_scene.objects[selected_object].rotation[1]).speed(0.1));  
                    ui.add(egui::DragValue::new(&mut reference.current_scene.objects[selected_object].rotation[2]).speed(0.1));
                } 
            }
            
         });

         if ui.button("Delete").clicked() {
            if let Some(selected_object) = reference.selected_object {
                    reference.current_scene.objects.remove(selected_object);
                    if Some(reference.selected_object) == Some(reference.rotation_index) {
                        reference.rotation_index = None;
                    }
                    reference.selected_object = None;
            }
    }
    });
    ui.add_space(10.0);
    ui.separator();
    ui.add_space(4.0);
    if reference.rotation_index == reference.selected_object {
    ui.vertical(|ui| {
        ui.add(TextEdit::singleline(&mut "X Rotation Speed").desired_width(110.0));
        ui.add(egui::Slider::new(&mut reference.speed_slider.0, 0.0..=100.0));

        if let Some(selected_object) = reference.selected_object {
            if selected_object < reference.current_scene.objects.len() {
                if ui.button("Reset").clicked() {
                    reference.current_scene.objects[selected_object].rotation[0] = 0.0;
                    reference.speed_slider.0 = 0.0;
                }
            }
        }
        

    });
    ui.vertical(|ui| {
        ui.add(TextEdit::singleline(&mut "Y Rotation Speed").desired_width(110.0));
        ui.add(egui::Slider::new(&mut reference.speed_slider.1, 0.0..=100.0));

        if let Some(selected_object) = reference.selected_object {
            if selected_object < reference.current_scene.objects.len() {
                if ui.button("Reset").clicked() {
                    reference.current_scene.objects[selected_object].rotation[1] = 0.0;
                    reference.speed_slider.1 = 0.0;
                }
            }
        }
        

    });
    ui.vertical(|ui| {
        ui.add(TextEdit::singleline(&mut "Z Rotation Speed").desired_width(110.0));
        ui.add(egui::Slider::new(&mut reference.speed_slider.2, 0.0..=100.0));

        if let Some(selected_object) = reference.selected_object {
            if selected_object < reference.current_scene.objects.len() {
                if ui.button("Reset").clicked() {
                    reference.current_scene.objects[selected_object].rotation[2] = 0.0;
                    reference.speed_slider.2 = 0.0;
                }
            }
        }
        
    });
    }
    let mut checked = reference.rotation_index == reference.selected_object;
    if ui.checkbox(&mut checked, "Current Rotating").changed() {
        if checked {
            reference.rotation_index = reference.selected_object;
        }
        else {
            reference.rotation_index = None;
        }
    };
ui.add_space(10.0);
}

fn gerneral_settings(ui: &mut Ui, reference : &mut Content, deltaTime: f32) {
    ui.set_min_width(0.0);
    ui.add(TextEdit::singleline(&mut "Light Settings:").desired_width(110.0));
    ui.add_space(10.0);


        ui.add(TextEdit::singleline(&mut "Light Intensity:").desired_width(110.0));
        ui.add(egui::DragValue::new(&mut reference.current_scene.light.intensity).speed(0.1));  

        
    ui.add_space(4.0);

        ui.add(TextEdit::singleline(&mut "Light Position:").desired_width(110.0));

        ui.horizontal(|ui| {
        ui.add(egui::DragValue::new(&mut reference.current_scene.light.position[0]).speed(0.1));  
        ui.add(egui::DragValue::new(&mut reference.current_scene.light.position[1]).speed(0.1));  
        ui.add(egui::DragValue::new(&mut reference.current_scene.light.position[2]).speed(0.1));  
        });   
}

fn camera_settings(ui: &mut Ui, reference : &mut Content, deltaTime: f32) {
    ui.add(TextEdit::singleline(&mut "Camera Settings:").desired_width(110.0));
    ui.add_space(10.0);

    ui.add(TextEdit::singleline(&mut "Position:").desired_width(110.0));
    ui.add_space(4.0);

    ui.horizontal(|ui| {
        ui.add(egui::DragValue::new(&mut reference.current_scene.camera_position[0]).speed(0.05));  
        ui.add(egui::DragValue::new(&mut reference.current_scene.camera_position[1]).speed(0.05));  
        ui.add(egui::DragValue::new(&mut reference.current_scene.camera_position[2]).speed(0.05));  
    });

    ui.add(TextEdit::singleline(&mut "Rotation:").desired_width(110.0));
    ui.add_space(4.0);

    ui.horizontal(|ui| {
        ui.add(egui::DragValue::new(&mut reference.current_scene.camera_rotation[0]).speed(0.05));  
        ui.add(egui::DragValue::new(&mut reference.current_scene.camera_rotation[1]).speed(0.05));  
        ui.add(egui::DragValue::new(&mut reference.current_scene.camera_rotation[2]).speed(0.05));  
    });
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