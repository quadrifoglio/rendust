use std;
use std::os::raw::c_void;

use gl;
use gl::types::*;

use super::Color;

// Define a globally available default blank texture
lazy_static! {
    pub static ref BlankTexture: Texture = Texture::blank();
}

/// Represents a 3D vertex
#[repr(C)]
pub struct Vertex {
    pub position: [GLfloat; 3],
    pub color: Color,
    pub texcoords: [GLfloat; 2]
}

impl Vertex {
    /// Create a new colored vertex
    pub fn new(pos: [GLfloat; 3], c: Color) -> Vertex {
        Vertex {
            position: pos,
            color: c,
            texcoords: [0.0, 0.0]
        }
    }

    /// Create a new textured vertex
    pub fn textured(pos: [GLfloat; 3], tex: [GLfloat; 2]) -> Vertex {
        Vertex {
            position: pos,
            color: [1.0, 1.0, 1.0, 1.0],
            texcoords: tex
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
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

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

    /// Create a white texture to be used when
    /// no other texture is loaded
    pub fn blank() -> Texture {
        Texture::new(1, 1, &[255, 255, 255, 255])
    }

    /// Bind the texture for use in
    /// rendering
    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    /// Unbind the texture
    /// This actually binds a default blank texture
    pub fn unbind(&self) {
        BlankTexture.bind();
    }
}

/// Represents all the drawable primitives
/// that can be rendered to the screen
pub enum PrimitiveType {
    Points,
    Lines,
    Triangles,
    Quads
}

impl PrimitiveType {
    // Get the corresponding GL constant
    fn to_gl_const(&self) -> GLuint {
        match *self {
            PrimitiveType::Points => gl::POINTS,
            PrimitiveType::Lines => gl::LINES,
            PrimitiveType::Triangles => gl::TRIANGLES,
            PrimitiveType::Quads => gl::QUADS
        }
    }
}

/// Represents a renderable 3D object
pub struct Mesh {
    primitive: PrimitiveType,
    count: i32,
    vbo: GLuint,
    ibo: Option<GLuint>
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

            // Optional IBO
            let mut ibo: Option<GLuint> = None;
            let mut count = vertices.len() as i32;

            // If the caller specified indicies
            if let Some(indicies) = indicies {
                // Create a new IBO
                let mut ibo_id: GLuint = 0;

                gl::GenBuffers(1, (&mut ibo_id) as *mut GLuint);
                gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ibo_id);

                // Upload indicies to the graphics card
                gl::BufferData(
                    gl::ELEMENT_ARRAY_BUFFER,
                    (std::mem::size_of::<u32>() * indicies.len()) as GLsizeiptr,
                    indicies.as_ptr() as *const c_void,
                    gl::STATIC_DRAW
                );

                // Unbind the IBO
                gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);

                // Specify the IBO and use the correct element count
                ibo = Some(ibo_id);
                count = indicies.len() as i32;
            }

            Mesh{
                primitive: p,
                count: count,
                vbo: vbo,
                ibo: ibo
            }
        }
    }

    /// Render the mesh to the screen
    pub fn render(&self) {
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            // Bind the VBO
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);

            // Enable the atttributes
            gl::EnableVertexAttribArray(0);
            gl::EnableVertexAttribArray(1);
            gl::EnableVertexAttribArray(2);

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

            // Texture coordinates attribute
            // Offsert in vertex memory structure: 3 floats + 4 floats, 28 bytes
            gl::VertexAttribPointer(
                2 as GLuint,
                2 as GLint,
                gl::FLOAT,
                gl::FALSE,
                std::mem::size_of::<Vertex>() as GLsizei,
                (std::mem::size_of::<f32>() * (3 + 4)) as *const c_void
            );

            // If indicies are being used
            if let Some(ibo) = self.ibo {
                gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ibo);
                gl::DrawElements(self.primitive.to_gl_const(), self.count, gl::UNSIGNED_INT, 0 as *const c_void);
                gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
            }
            // Otherwise just render the vertices
            else {
                gl::DrawArrays(self.primitive.to_gl_const(), 0, self.count);
            }

            // Disable the attributes
            gl::DisableVertexAttribArray(0);
            gl::DisableVertexAttribArray(1);
            gl::DisableVertexAttribArray(2);

            // Unbind VBO
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }
}
