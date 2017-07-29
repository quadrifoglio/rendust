/// Represents an ambient light source
pub struct Ambient {
    pub color: [f32; 4],
    pub strength: f32
}

impl Ambient {
    /// Create a new ambient light
    pub fn new(color: [f32; 4], strength: f32) -> Ambient {
        Ambient {
            color: color,
            strength: strength
        }
    }
}
