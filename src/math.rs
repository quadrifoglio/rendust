use cgmath::{self, Matrix4};

/// Crate a perspective projection matrix
/// FoV angle needs to be specified in degrees
pub fn perspective(fovy: f32, aspect: f32, near: f32, far: f32) -> Matrix4<f32> {
    cgmath::perspective(cgmath::Deg(fovy), aspect, near, far)
}
