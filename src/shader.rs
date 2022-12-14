use std::ffi::CString;

use gl::types::*;

use crate::math::Matrix4;

pub enum ShaderType {
    Vertex = gl::VERTEX_SHADER as isize,
    Fragment = gl::FRAGMENT_SHADER as isize,
}

pub struct Shader(pub GLuint);
impl Shader {
    pub fn new(shader_type: ShaderType) -> Option<Self> {
        let shader = unsafe { gl::CreateShader(shader_type as GLenum) };

        if shader != 0 {
            Some(Self(shader))
        } else {
            None
        }
    }

    //Add source code to the shader object
    pub fn set_source(&self, src: &str) {
        unsafe {
            gl::ShaderSource(
                self.0,
                1,
                &(src.as_bytes().as_ptr().cast()), //Specifies pointer to source code
                &(src.len().try_into().unwrap()),
            );
        }
    }

    pub fn compile(&self) {
        unsafe { gl::CompileShader(self.0) }
    }

    pub fn compile_success(&self) -> bool {
        let mut success = 0;

        unsafe { gl::GetShaderiv(self.0, gl::COMPILE_STATUS, &mut success) }

        success == i32::from(gl::TRUE)
    }

    //Automatically creates and compiles shader based on type and source code params
    pub fn create_shader_from_src(shader_type: ShaderType, src: &str) -> Result<Self, String> {
        let shader =
            Self::new(shader_type).ok_or_else(|| "Couldn't create new shader".to_string())?;
        shader.set_source(src);
        shader.compile();

        if shader.compile_success() {
            Ok(shader)
        } else {
            //Failed to compile shader
            let output = format!("Failed to compile shader: {}", shader.get_error_log());
            shader.delete();
            Err(output)
        }
    }

    pub fn delete(&self) {
        unsafe { gl::DeleteShader(self.0) }
    }

    pub fn get_error_log(&self) -> String {
        //we dont get compile status bc we get that in compile_success
        let mut length = 0;

        unsafe { gl::GetShaderiv(self.0, gl::INFO_LOG_LENGTH, &mut length) }

        let mut v: Vec<u8> = Vec::with_capacity(length.try_into().unwrap());
        let mut length_written = 0_i32;

        unsafe {
            gl::GetShaderInfoLog(
                self.0,
                v.capacity().try_into().unwrap(),
                &mut length_written,
                v.as_mut_ptr().cast(),
            );

            v.set_len(length_written.try_into().unwrap());
        }

        String::from_utf8_lossy(&v).into_owned()
    }
}

pub struct ShaderProgram(pub GLuint);
impl ShaderProgram {
    pub fn new() -> Option<Self> {
        let program = unsafe { gl::CreateProgram() };

        if program != 0 {
            Some(Self(program))
        } else {
            None
        }
    }

    //Attach shader object to this program object
    pub fn attach_shader(&self, shader: &Shader) {
        unsafe { gl::AttachShader(self.0, shader.0) }
    }

    pub fn link_program(&self) {
        unsafe { gl::LinkProgram(self.0) }
    }

    pub fn link_success(&self) -> bool {
        let mut success = 0;
        unsafe { gl::GetProgramiv(self.0, gl::LINK_STATUS, &mut success) }
        success == i32::from(gl::TRUE)
    }

    pub fn use_program(&self) {
        unsafe { gl::UseProgram(self.0) }
    }

    pub fn delete(&self) {
        unsafe { gl::DeleteProgram(self.0) }
    }

    //Creates the program using vertex and fragment SOURCE CODE
    pub fn create_program_from_src(vert: &str, frag: &str) -> Result<Self, String> {
        let program = Self::new().ok_or_else(|| "Failed to create program".to_string())?;

        let vertex = Shader::create_shader_from_src(ShaderType::Vertex, vert)
            .map_err(|e| format!("Vertex shader failed to compile: {}", e))?;

        let fragment = Shader::create_shader_from_src(ShaderType::Fragment, frag)
            .map_err(|e| format!("Fragment shader failed to compile: {}", e))?;

        program.attach_shader(&vertex);
        program.attach_shader(&fragment);
        program.link_program();

        vertex.delete();
        fragment.delete();

        if program.link_success() {
            Ok(program)
        } else {
            let output = format!("Failed to link program: {}", program.get_error_log());
            program.delete();
            Err(output)
        }
    }

    pub fn create_program_from_files(vert_path: &str, frag_path: &str) -> Result<Self, String> {
        //Read get src from files
        let vert_src = std::fs::read_to_string(vert_path).unwrap_or_else(|_| {
            panic!(
                "Failed to read Vertex Shader source from path: {}",
                vert_path
            )
        });

        let frag_src = std::fs::read_to_string(frag_path).unwrap_or_else(|_| {
            panic!(
                "Failed to read Fragment Shader source from path: {}",
                frag_path
            )
        });

        let program = Self::create_program_from_src(&vert_src, &frag_src)
            .map_err(|e| format!("Failed to create program from files: {}", e));

        program
    }

    pub fn get_error_log(&self) -> String {
        //we dont get compile status bc we get that in compile_success
        let mut length = 0;

        unsafe { gl::GetProgramiv(self.0, gl::INFO_LOG_LENGTH, &mut length) }

        let mut v: Vec<u8> = Vec::with_capacity(length.try_into().unwrap());
        let mut length_written = 0_i32;

        unsafe {
            gl::GetShaderInfoLog(
                self.0,
                v.capacity().try_into().unwrap(),
                &mut length_written,
                v.as_mut_ptr().cast(),
            );

            v.set_len(length_written.try_into().unwrap());
        }

        String::from_utf8_lossy(&v).into_owned()
    }

    pub fn get_uniform_location(&self, name: &str) -> Option<i32> {
        let c_name = CString::new(name).unwrap();
        let location: i32;

        unsafe {
            location = gl::GetUniformLocation(self.0, c_name.as_ptr());
        };

        if location != -1 {
            Some(location)
        } else {
            None
        }
    }
}

// TODO: generalize this function (aka, make it uniform instead of uniform 4f
// and let them enter what they want)
pub fn uniform4f(location: i32, r: f32, g: f32, b: f32, a: f32) {
    unsafe { gl::Uniform4f(location, r, g, b, a) }
}

pub fn uniform_matrix_4fv(location: i32, data: Matrix4) {
    unsafe { gl::UniformMatrix4fv(location, 1, gl::FALSE, data.to_ptr()) }
}
