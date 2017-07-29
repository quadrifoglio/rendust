// External libraries
#[macro_use]
extern crate lazy_static;

extern crate gl;
extern crate libc;
extern crate glutin;
extern crate cgmath;

use std::io;

/// Error type used in this library
#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    WindowCreation(glutin::CreationError),
    GlContext(glutin::ContextError),
    GlShader(String)
}

/// Result type used in this library
pub type Result<T> = std::result::Result<T, Error>;

// Root scope types and functions

/// Set the clearing color
pub fn set_clear_color(r: f32, g: f32, b: f32, a: f32) {
    unsafe {
        gl::ClearColor(r, g, b, a);
    }
}

/// Clear the screen, both the color buffer
/// and the depth buffer
pub fn clear() {
    unsafe {
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    }

    // Bind a default blank texture, in case no other
    // texture is being use
    mesh::BLANK_TEXTURE.bind();
}

// Windowing subsystem
pub mod window;

// Shader program management
pub mod shaders;

// Mesh rendering
pub mod mesh;

// Math functionality
pub mod math;

// Camera
pub mod camera;
