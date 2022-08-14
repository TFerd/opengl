use std::ffi::c_void;

use gl::types::*;

pub enum TextureType {
    Texture2D = gl::TEXTURE_2D as isize,
    Texture3D = gl::TEXTURE_3D as isize,
}

pub enum TextureOption {
    //GL_TEXTURE_WRAP_S, GL_TEXTURE_WRAP_T, or GL_TEXTURE_WRAP_R.
    TextureWrapR = gl::TEXTURE_WRAP_R as isize,
    TextureWrapS = gl::TEXTURE_WRAP_S as isize,
    TextureWrapT = gl::TEXTURE_WRAP_T as isize,
    MinFilter = gl::TEXTURE_MIN_FILTER as isize,
    MagFilter = gl::TEXTURE_MAG_FILTER as isize,
}

pub enum TextureOptionValue {
    Repeat = gl::REPEAT as isize,
    MirroredRepeat = gl::MIRRORED_REPEAT as isize,
    ClampToEdge = gl::CLAMP_TO_EDGE as isize,
    ClampToBorder = gl::CLAMP_TO_BORDER as isize,
    Nearest = gl::NEAREST as isize,
    Linear = gl::LINEAR as isize,
}

pub enum InternalFormat {
    RG = gl::RG as isize,
    RGB = gl::RGB as isize,
    RGBA = gl::RGBA as isize,
    RED = gl::RED as isize,
    DepthComponent = gl::DEPTH_COMPONENT as isize,
    DepthStencil = gl::DEPTH_STENCIL as isize,
}

// pub enum TexParameter postfix?
pub struct Texture(pub GLuint);
impl Texture {
    pub fn new() -> Option<Self> {
        let mut texture = 0;
        unsafe { gl::GenTextures(1, &mut texture) };

        if texture != 0 {
            Some(Self(texture))
        } else {
            None
        }
    }

    pub fn bind(&self, ty: TextureType) {
        unsafe { gl::BindTexture(ty as GLenum, self.0) };
    }
}
// can i find out which texture is bound so i can determine if i can bind it, make it more based on the object ya know
// so like maybe the object gets a TextureType property
// Assigns wrap_mode to option
pub fn set_parameter(target: TextureType, parameter: TextureOption, value: TextureOptionValue) {
    // @TODO: add more function calls, AKA:: pub enum TexParameter postfix?
    unsafe { gl::TexParameteri(target as GLenum, parameter as GLenum, value as GLint) }
}

pub fn generate_mipmap(ty: TextureType) {
    unsafe { gl::GenerateMipmap(ty as GLenum) }
}

// specify a 2D texture image
pub fn tex_image_2d(
    tex_type: TextureType,
    mipmap_lvl: i32,
    internal_format: InternalFormat,
    width: u32,
    height: u32,
    format: InternalFormat,
    data: &u8, //This must passed in like &VecU8[0]
) {
    unsafe {
        gl::TexImage2D(
            tex_type as GLenum,
            mipmap_lvl,
            internal_format as GLint,
            width as i32,
            height as i32,
            0,
            format as GLenum,
            gl::UNSIGNED_BYTE,
            data as *const u8 as *const c_void,
        )
    }
}
