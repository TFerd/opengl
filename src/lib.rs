//Wrapper for the gl wrapper (lol) for OpenGL
//@TODO: Make a window class
extern crate gl;
extern crate glfw;

use gl::types::*;

pub mod buffer;
pub mod error_handling;
pub mod math;
pub mod shader;
pub mod texture;
pub mod vertex_array;
pub mod window;

pub fn clear_color(r: f32, g: f32, b: f32, a: f32) {
    unsafe {
        gl::ClearColor(r, g, b, a);
    }
}

pub enum PolygonMode {
    //Show only points
    Point = gl::POINT as isize,

    //Show only lines
    Line = gl::LINE as isize,

    //Fill in polygons
    Fill = gl::FILL as isize,
}

pub fn polygon_mode(mode: PolygonMode) {
    unsafe { gl::PolygonMode(gl::FRONT_AND_BACK, mode as GLenum) }
}
