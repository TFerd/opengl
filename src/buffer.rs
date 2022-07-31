use gl::types::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferType {
    // Array Buffers holds arrays of vertex data for drawing.
    Array = gl::ARRAY_BUFFER as isize,

    // Element Array Buffers hold indexes of what vertexes to use for drawing.
    ElementArray = gl::ELEMENT_ARRAY_BUFFER as isize,
}

pub struct Buffer(pub GLuint);
impl Buffer {
    pub fn new() -> Option<Self> {
        let mut vbo = 0;
        unsafe { gl::GenBuffers(1, &mut vbo) }

        if vbo != 0 {
            Some(Self(vbo))
        } else {
            None
        }
    }

    pub fn bind(&self, buffer_type: BufferType) {
        unsafe { gl::BindBuffer(buffer_type as GLenum, self.0) }
    }

    pub fn clear_binding(buffer_type: BufferType) {
        unsafe { gl::BindBuffer(buffer_type as GLenum, 0) }
    }

    pub fn buffer_data(buffer_type: BufferType, data: &[u8], usage: GLenum) {
        unsafe {
            gl::BufferData(
                buffer_type as GLenum,
                data.len().try_into().unwrap(),
                data.as_ptr().cast(),
                usage,
            )
        }
    }
}
