mod compute_shader;
mod pipeline_shader;

pub use base::{compile_shader, link_program, string_from_shader_source, ShaderSource};
pub use compute_shader::ComputeShader;
pub use pipeline_shader::PipelineShader;

mod base {
    /// Enables loading shaders from either a file or a hardcoded str
    pub enum ShaderSource<'a> {
        File(&'a str),
        String(&'a str),
    }

    /// Loads a given shader source.
    /// Can only fail if the shader source is a file which cannot be read.
    pub fn string_from_shader_source(
        source: Option<ShaderSource>,
    ) -> Result<Option<String>, Box<dyn std::error::Error>> {
        Ok(match source {
            Some(source) => match source {
                ShaderSource::File(file_path) => Some(std::fs::read_to_string(file_path)?),
                ShaderSource::String(source_code) => Some(String::from(source_code)),
            },
            None => None,
        })
    }

    pub fn compile_shader(source: &String, shader: u32) -> Result<(), Box<dyn std::error::Error>> {
        unsafe {
            let ptr: *const u8 = source.as_bytes().as_ptr();
            let ptr: *const i8 = std::mem::transmute(ptr);
            let len = source.len() as gl::types::GLint;

            gl::ShaderSource(shader, 1, &ptr, &len);

            gl::CompileShader(shader);

            let mut result = 0 as gl::types::GLint;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut result);

            if result == 0 {
                Err(compilation_log(shader)?)?
            }
        }

        Ok(())
    }

    /// Gathers the given shaders compilation log as a Rust string
    fn compilation_log(shader: u32) -> Result<String, Box<dyn std::error::Error>> {
        let mut len = 0;
        unsafe { gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len) };
        if len <= 0 {
            Err("Can't access shader compilation log")?
        }

        let mut buffer = Vec::with_capacity(len as usize);
        let buffer_ptr = buffer.as_mut_ptr() as *mut gl::types::GLchar;
        unsafe {
            gl::GetShaderInfoLog(shader, len, std::ptr::null_mut(), buffer_ptr);
            buffer.set_len(len as usize);
        }

        match String::from_utf8(buffer) {
            Ok(log) => Ok(log),
            Err(e) => Err(e)?,
        }
    }

    pub fn link_program(program: u32) -> Result<(), Box<dyn std::error::Error>> {
        unsafe {
            gl::LinkProgram(program);

            let mut result = 0;
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut result);
            if result == 0 {
                Err(link_log(program)?)?
            }
        }

        Ok(())
    }

    /// Gathers the given programs linking log as a Rust string
    fn link_log(program: u32) -> Result<String, Box<dyn std::error::Error>> {
        let mut len = 0;
        unsafe { gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len) }
        if len <= 0 {
            Err("Can't access program link log")?
        }

        let mut buffer = Vec::with_capacity(len as usize);
        let buffer_ptr = buffer.as_mut_ptr() as *mut gl::types::GLchar;
        unsafe {
            gl::GetProgramInfoLog(program, len, std::ptr::null_mut(), buffer_ptr);
            buffer.set_len(len as usize);
        }

        match String::from_utf8(buffer) {
            Ok(log) => Ok(log),
            Err(e) => Err(e)?,
        }
    }
}
