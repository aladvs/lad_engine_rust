# Lad Engine Rust
A 3D rasterizer with lighting, written entirely in Rust and using egui.

> I created this as an introduction to Rust, so I assume there will be a few issues with how my code is structured.
> Yes, this should be split into multiple files. My bad.

## Web Demo
Check out the [Web Demo](https://aladvs.github.io/Lad-Engine-Rust-web).


![Web Demo](https://github.com/aladvs/lad_engine_rust/assets/78510667/6c197875-8f97-4c71-91c9-fe74b03f0cc2)


### How to Use
#### Scene View:
There is a scene hierarchy with all objects in the scene. Click on one to edit its attributes or to delete it.

![Scene View](https://github.com/aladvs/lad_engine_rust/assets/78510667/af932888-2f58-4ca5-aea5-43339f45f0b4)


#### Rotation:
You can set one object to be spinning; this is hardcoded. You can have either none spinning or one object, and you can change the speed.

![Rotation](https://github.com/aladvs/lad_engine_rust/assets/78510667/8d7c3eb4-0000-42b5-b71d-b058ac883118)


#### Importing OBJ Files:
You can import an .obj file by dragging and dropping the file into the window. Keep in mind that only triangulated meshes are currently supported.

#### Lighting & Camera Settings:
You can edit the light's intensity and light position (currently only one light is supported, but adding another light should be straightforward by doing another pass and then adding the two results).

You can also change the camera's position and rotation (or move using WASD and the left and right arrow keys to rotate).

![Lighting & Camera Settings](https://github.com/aladvs/lad_engine_rust/assets/78510667/4913c555-3b73-411c-9389-c8d0581408ec)
