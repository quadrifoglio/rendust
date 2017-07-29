use shaders::Program;

/// Represents an ambient light source
pub struct Ambient {
    color: [f32; 4],
    strength: f32
}

impl Ambient {
    /// Create a new ambient light
    pub fn new(color: [f32; 4], strength: f32) -> Ambient {
        Ambient {
            color: color,
            strength: strength
        }
    }

    /// Apply the light
    pub fn apply(&self, program: &Program) {
        program.set_uniform_vector("ambient_light_color", 4, &self.color);
        program.set_uniform_float("ambient_light_strength", self.strength);
    }
}
