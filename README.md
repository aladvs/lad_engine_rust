# Lad Engine Rust
A 3D rasterizer with lighting, written entirely in Rust and using egui.

> I created this as an introduction to Rust, so I assume there will be a few issues with how my code is structured.

## Web Demo
Check out the [Web Demo](https://aladvs.github.io/Lad-Engine-Rust-web).


![Web Demo](https://github.com/aladvs/lad_engine_rust/assets/78510667/6bc4a0c0-0dc2-456b-ab3e-77b33073facc)

### How to Use
#### Scene View:
There is a scene hierarchy with all objects in the scene. Click on one to edit its attributes or to delete it.

![Scene View](https://github.com/aladvs/lad_engine_rust/assets/78510667/349585ef-8f38-493d-95b3-60639b18e55e)

#### Rotation:
You can set one object to be spinning; this is hardcoded. You can have either none spinning or one object, and you can change the speed.

![Rotation](https://github.com/aladvs/lad_engine_rust/assets/78510667/5fc6b1e0-40b1-4e08-8831-f316af050050)

#### Importing OBJ Files:
You can import an .obj file by dragging and dropping the file into the window. Keep in mind that only triangulated meshes are currently supported.

#### Lighting & Camera Settings:
You can edit the light's intensity and light position (currently only one light is supported, but adding another light should be straightforward by doing another pass and then adding the two results).

You can also change the camera's position and rotation.

![Lighting & Camera Settings](https://github.com/aladvs/lad_engine_rust/assets/78510667/ba63197c-852e-4048-bd66-74ce6465c5de)
