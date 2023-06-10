extern crate gl;

/*
* Module with implementation of Shader and Shader Program abstraction for OpenGL
*/

use std::{fs};
use std::path::Path;
use std::ffi::{CString};

///Struct encapsulating Shader Program ID
pub struct Program {
    program_id : gl::types::GLuint,
}

impl Program {
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

            return Err(error.to_string_lossy().into_owned())
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
    ///Creates shader with [`shader_type`] from source file stored in [`filepath`]
    pub fn from_source_file(filepath: &str, shader_type: gl::types::GLenum) -> Result<Shader, String> {
        let source = load_file(filepath).expect("Failed to load shader source");

        let id = unsafe { gl::CreateShader(shader_type) };

        let cstr = CString::new(source).expect("Failed to convert shader source to CString");

        unsafe {
            gl::ShaderSource(id, 1,  &cstr.as_c_str().as_ptr(), std::ptr::null());
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

fn load_file(filepath : &str) -> Result<String, String> {
    let path = Path::new(filepath);

    println!("Reading file from {}...", {path.display()});

    match fs::read_to_string(path) {
        Ok(res) => return Ok(res),
        Err(err) => {
            return Err(err.to_string());
        } 
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
