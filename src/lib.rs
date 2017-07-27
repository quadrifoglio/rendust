// External libraries
extern crate gl;
extern crate libc;
extern crate glutin;

use std::io;

/// Error type used in this library
pub enum Error {
    IoError(io::Error),
    WindowCreationError(glutin::CreationError),
    GlContextError(glutin::ContextError)
}

/// Result type used in this library
pub type Result<T> = std::result::Result<T, Error>;

// Windowing subsystem
pub mod window;
