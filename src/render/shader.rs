extern crate gl;

/*
* Module with implementation of Shader and Shader Program abstraction for OpenGL
*/
use std::ffi::{CString};
use crate::resources::{self, Resources};

#[derive(Debug)]
pub enum Error {
    ResourceLoad { name:String, inner: resources::Error},
    CanNotDetermineShaderTypeForResource { name: String },
    CompileError { name: String, message: String },
    LinkError { name: String, message: String },
}

///Struct encapsulating Shader Program ID
pub struct Program {
    program_id : gl::types::GLuint,
}

impl Program {
    pub fn from_resources(res: &Resources, name: &str) -> Result<Program, Error> {
        const POSSIBLE_EXTENSION: [&str; 2] = [".vert", ".frag"];

        let resources_names = POSSIBLE_EXTENSION
            .iter()
            .map(|file_extension| format!("{}{}", name, file_extension))
            .collect::<Vec<String>>();

        let shaders = resources_names
            .iter()
            .map(|resource_name| Shader::from_resources(res, resource_name))
            .collect::<Result<Vec<Shader>, Error>>()?;

        Program::from_shaders(&shaders[..]).map_err(|message| Error::LinkError { 
            name: name.into(), 
            message
        })
    }

    /// Creates program based of [`shaders`] slice, all provided shaders
    /// will be linked into program
    pub fn from_shaders(shaders: &[Shader]) -> Result<Program, String>{
        let id = unsafe { gl::CreateProgram() };

        for shader in shaders {
            unsafe { 
                gl::AttachShader(id, shader.id());
            }
        }

        unsafe { gl::LinkProgram(id); }

        let mut success: gl::types::GLint = 1;
        unsafe {
            gl::GetProgramiv(id, gl::COMPILE_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len);      
            }

            let error = create_whitespace_cstring_with_len(len as usize);

            unsafe {
                gl::GetProgramInfoLog(id, len, std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar);
            }

            return Err( error.to_string_lossy().into_owned() )
        }

        for shader in shaders {
            unsafe { 
                gl::DetachShader(id, shader.id());
            }
        }

        return Ok(Program { program_id: id });
    }

    pub fn set_used(&self) {
        unsafe {
            gl::UseProgram(self.program_id);
        }
    }

    pub fn id(&self) -> gl::types::GLuint {
        return self.program_id;
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.program_id);
        }
    }
}

pub struct Shader {
    pub shader_id : gl::types::GLuint,
}

impl Shader {
    pub fn from_resources(res: &Resources, name: &str) ->Result<Shader, Error> {
        const POSSIBLE_EXTENSION: [(&str, gl::types::GLenum); 2] = [
            (".vert", gl::VERTEX_SHADER),
            (".frag", gl::FRAGMENT_SHADER)
        ];

        let shader_kind = POSSIBLE_EXTENSION.iter().find(|&&(file_extension, _)| {
            name.ends_with(file_extension)
        }).map(|&(_, kind)| kind).ok_or_else(|| Error::CanNotDetermineShaderTypeForResource { name: name.into() })?;

        let source = res.load_cstring(name).map_err(|e| Error::ResourceLoad {
            name: name.into(),
            inner: e,
        })?;

        Shader::from_source(&source, shader_kind).map_err(|message| Error::CompileError { 
            name: name.into(), 
            message, 
        })
    }

    ///Creates shader with [`shader_type`] from source file stored in [`filepath`]
    pub fn from_source(source: &CString, shader_type: gl::types::GLenum) -> Result<Shader, String> {
        let id = unsafe { gl::CreateShader(shader_type) };

        unsafe {
            gl::ShaderSource(id, 1,  &source.as_c_str().as_ptr(), std::ptr::null());
            gl::CompileShader(id);
        }

        let mut success: gl::types::GLint = 1;
        unsafe {
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);      
            }

            let error = create_whitespace_cstring_with_len(len as usize);

            unsafe {
                gl::GetShaderInfoLog(id, len, std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar);
            }

            return Err(error.to_string_lossy().into_owned())
        }

        return Ok(Shader { shader_id: id })
    }

    pub fn id(&self) -> gl::types::GLuint {
        return self.shader_id;
    }

}

///create buffer with specified [`len`]
fn create_whitespace_cstring_with_len(len: usize) -> CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    buffer.extend([b' '].iter().cycle().take(len));
    unsafe { CString::from_vec_unchecked(buffer) }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.shader_id);
        }
    }
}
