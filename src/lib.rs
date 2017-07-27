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

// Windowing subsystem
pub mod window;

// Shader program management
pub mod shaders;
