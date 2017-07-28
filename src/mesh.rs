use std;
use std::os::raw::c_void;

use gl;
use gl::types::*;

use super::Color;

/// Represents a 3D vertex
#[repr(C)]
pub struct Vertex {
    pub position: [GLfloat; 3],
    pub color: Color
}

impl Vertex {
    /// Create a new vertex
    pub fn new(pos: [GLfloat; 3], c: Color) -> Vertex {
        Vertex {
            position: pos,
            color: c
        }
    }
}

/// Represents a texture
/// A 2D image loaded onto the graphics card's memory
pub struct Texture {
    pub width: u32,
    pub height: u32,

    id: GLuint
}

impl Texture {
    /// Create a new texture with the specified with and height.
    /// The image data must be 8 bit RGBA
    pub fn new(width: u32, height: u32, data: &[u8]) -> Texture {
        unsafe {
            // Create and bind the texture
            let mut id: GLuint = 0;

            gl::GenTextures(1, (&mut id) as *mut GLuint);
            gl::BindTexture(gl::TEXTURE_2D, id);

            // Usual texture parameters
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR);

            // Upload the texture data to the GPU
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as GLint,
                width as GLsizei,
                height as GLsizei,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                data.as_ptr() as *const c_void
            );

            // Generate mipmap
            gl::GenerateMipmap(gl::TEXTURE_2D);

            // Unbind the texture
            gl::BindTexture(gl::TEXTURE_2D, 0);

            Texture {
                width: width,
                height: height,
                id: id
            }
        }
    }
}

/// Represents all the drawable primitives
/// that can be rendered to the screen
pub enum PrimitiveType {
    Points,
    Triangles
}

/// Represents a renderable 3D object
pub struct Mesh {
    primitive: PrimitiveType,
    vertex_count: i32,
    vbo: GLuint
}

impl Mesh {
    /// Create a new mesh
    pub fn new(p: PrimitiveType, vertices: &[Vertex], indicies: Option<&[u32]>) -> Mesh {
        unsafe {
            // Create a VBO to store vertex data
            let mut vbo: GLuint = 0;

            gl::GenBuffers(1, (&mut vbo) as *mut GLuint);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            // Upload the vertex data to the craphics card
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (std::mem::size_of::<Vertex>() * vertices.len()) as GLsizeiptr,
                vertices.as_ptr() as *const c_void,
                gl::STATIC_DRAW
            );

            // Unbind VBO
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);

            Mesh{
                primitive: p,
                vbo: vbo,
                vertex_count: vertices.len() as i32
            }
        }
    }

    /// Render the mesh to the screen
    pub fn render(&self) {
        unsafe {
            // Bind the VBO
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);

            // Enable the atttributes
            gl::EnableVertexAttribArray(0);
            gl::EnableVertexAttribArray(1);

            // Specify where each different attirbute of each vertex
            // are in GPU memory

            // Position attribute
            // Offsert in vertex memory structure: 0
            gl::VertexAttribPointer(
                0 as GLuint,
                3 as GLint,
                gl::FLOAT,
                gl::FALSE,
                std::mem::size_of::<Vertex>() as GLsizei,
                0 as *const c_void
            );

            // Color attribute
            // Offsert in vertex memory structure: 3 floats, 12 bytes
            gl::VertexAttribPointer(
                1 as GLuint,
                4 as GLint,
                gl::FLOAT,
                gl::FALSE,
                std::mem::size_of::<Vertex>() as GLsizei,
                (std::mem::size_of::<f32>() * 3) as *const c_void
            );

            // Render
            match self.primitive {
                PrimitiveType::Points => gl::DrawArrays(gl::POINTS, 0, self.vertex_count),
                PrimitiveType::Triangles => gl::DrawArrays(gl::TRIANGLES, 0, self.vertex_count),
            }

            // Disable the attributes
            gl::DisableVertexAttribArray(0);
            gl::DisableVertexAttribArray(1);

            // Unbind VBO
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }
}
