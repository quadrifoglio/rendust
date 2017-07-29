use cgmath;

/// 2D point
pub type P2 = cgmath::Point2<f32>;

/// 3D point
pub type P3 = cgmath::Point3<f32>;

/// 2D vector
pub type Vec2 = cgmath::Vector2<f32>;

/// 3D vector
pub type Vec3 = cgmath::Vector3<f32>;

/// 4x4 square matrix
pub type Mat4 = cgmath::Matrix4<f32>;

/// Degrees & Radians angles
pub use cgmath::Deg;
pub use cgmath::Rad;

pub use cgmath::SquareMatrix;

/// Crate a perspective projection matrix
/// FoV angle needs to be specified in degrees
pub fn perspective(fovy: f32, aspect: f32, near: f32, far: f32) -> Mat4 {
    cgmath::perspective(cgmath::Deg(fovy), aspect, near, far)
}
