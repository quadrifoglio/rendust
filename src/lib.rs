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

/// 2D vector
pub type Vec2 = cgmath::Vector2<f32>;

/// 3D vector
pub type Vec3 = cgmath::Vector3<f32>;

/// 4x4 square matrix
pub type Mat4 = cgmath::Matrix4<f32>;

/// Represents a color
/// RGBA, 4 32 bits floating point values
pub type Color = [f32; 4];

/// Set the clearing color
pub fn set_clear_color(c: Color) {
    unsafe {
        gl::ClearColor(c[0], c[1], c[2], c[3]);
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
    mesh::BlankTexture.bind();
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
