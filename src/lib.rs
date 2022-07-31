//Wrapper for the gl wrapper (lol) for OpenGL

extern crate gl;
extern crate glfw;

pub mod buffer;
pub mod error_handling;
pub mod shader;
pub mod vertex_array;

pub fn clear_color(r: f32, g: f32, b: f32, a: f32) {
    unsafe {
        gl::ClearColor(r, g, b, a);
    }
}
