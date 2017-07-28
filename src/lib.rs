// External libraries
extern crate gl;
extern crate libc;
extern crate glutin;

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

/// Represents a color
/// RGBA, 4 32 bits floating point values
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    /// Create a new color
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color {
            r: r,
            g: g,
            b: b,
            a: a
        }
    }
}

/// Set the clearing color
pub fn set_clear_color(c: Color) {
    unsafe {
        gl::ClearColor(c.r, c.g, c.b, c.a);
    }
}

/// Clear the screen, both the color buffer
/// and the depth buffer
pub fn clear() {
    unsafe {
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    }
}

// Windowing subsystem
pub mod window;

// Shader program management
pub mod shaders;

// Mesh rendering
pub mod mesh;
