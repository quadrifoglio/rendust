use cgmath::EuclideanSpace;

use math::{P3, Vec3, Mat4};

/// Represents a 3D camera
pub struct Camera {
    pub position: Vec3,
    pub target: Vec3,
    pub up: Vec3
}

impl Camera {
    /// Create a new camera. By default it will
    /// look towards the negative z
    pub fn new(p: Vec3) -> Camera {
        Camera {
            position: p,
            target: Vec3::new(0.0, 0.0, -1.0),
            up: Vec3::new(0.0, 1.0, 0.0)
        }
    }

    /// Compute the view matrix associated with
    /// this camera's current settings
    pub fn view_matrix(&self) -> Mat4 {
        Mat4::look_at(P3::from_vec(self.position), P3::from_vec(self.position + self.target), self.up)
    }
}
