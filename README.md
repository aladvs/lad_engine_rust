# Lad Engine Rust
3d rasterizer with lighting written entirely in Rust and using egui.

>I created this as an introduction to Rust, so I assume there will be a few issues with how my code is structured

## [Web Demo](https://aladvs.github.io/Lad-Engine-Rust/)

###How to use:

####Scene view:
![image](https://github.com/aladvs/lad_engine_rust/assets/78510667/349585ef-8f38-493d-95b3-60639b18e55e)
There is a scene heirarchy with all objects in the scene. Click on one to edit its attributes, or to delete it.
![image](https://github.com/aladvs/lad_engine_rust/assets/78510667/c021410a-fc56-4da1-badf-331aee1f07b4)

####Rotation:
You can set one object to be spinning, this is hardcoded. You can have either none spinning, or one object. You can change the speed.
![image](https://github.com/aladvs/lad_engine_rust/assets/78510667/5fc6b1e0-40b1-4e08-8831-f316af050050)

####Importing OBJ files:
You can import an .obj file by drag and dropping the file into the window. Keep in mind only triangulated meshes are currently supported.

####Lighting & Camera Settings:
You can edit the light's intensity and light position. (currently only one light is supported but it should be pretty easy by doing another pass for another light and then adding the two results.)

You can change the camera's position and rotation. 
![image](https://github.com/aladvs/lad_engine_rust/assets/78510667/ba63197c-852e-4048-bd66-74ce6465c5de)
