use std::ffi::CString;

use super::ShaderSource;

pub struct ComputeShader {
    id: u32,
}

impl Drop for ComputeShader {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.id) }
    }
}

impl ComputeShader {
    /// Constructs a compute shader from a given file
    pub fn create(source: ShaderSource) -> Result<ComputeShader, Box<dyn std::error::Error>> {
        let src = match source {
            ShaderSource::File(file_name) => std::fs::read_to_string(file_name)?,
            ShaderSource::String(source_code) => String::from(source_code),
        };

        let shader = ComputeShader {
            id: unsafe { gl::CreateShader(gl::COMPUTE_SHADER) },
        };

        unsafe {
            let ptr: *const u8 = src.as_bytes().as_ptr();
            let ptr: *const i8 = std::mem::transmute(ptr);
            let len = src.len() as gl::types::GLint;
            gl::ShaderSource(shader.id, 1, &ptr, &len);
        }

        let success = unsafe {
            gl::CompileShader(shader.id);

            let mut result: gl::types::GLint = 0;
            gl::GetShaderiv(shader.id, gl::COMPILE_STATUS, &mut result);
            result != 0
        };

        if !success {
            Err(shader.compilation_log()?)?
        }

        let program_id = unsafe { gl::CreateProgram() };

        unsafe {
            gl::AttachShader(program_id, shader.id);
            gl::LinkProgram(program_id);
            gl::DeleteShader(shader.id);
        }

        Ok(ComputeShader { id: program_id })
    }

    /// Gathers the compilation log, in case the shader cannot be compiled
    fn compilation_log(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut len = 0;
        unsafe { gl::GetShaderiv(self.id, gl::INFO_LOG_LENGTH, &mut len) };
        if len <= 0 {
            Err("Can't access shader compilation log")?
        }

        let mut buffer = Vec::with_capacity(len as usize);
        let buffer_ptr = buffer.as_mut_ptr() as *mut gl::types::GLchar;
        unsafe {
            gl::GetShaderInfoLog(self.id, len, std::ptr::null_mut(), buffer_ptr);
            buffer.set_len(len as usize);
        }

        match String::from_utf8(buffer) {
            Ok(log) => Ok(log),
            Err(e) => Err(e)?,
        }
    }

    /// Dispatch this compute shader
    pub fn dispatch(&self, num_groups_x: u32, num_groups_y: u32, num_groups_z: u32, barrier: u32) {
        unsafe {
            gl::UseProgram(self.id);
            gl::DispatchCompute(num_groups_x, num_groups_y, num_groups_z);
            gl::MemoryBarrier(barrier);
        }
    }

    pub fn set_uint(&self, var: &str, value: u32) {
        unsafe {
            let s = CString::new(var).unwrap();
            let loc = gl::GetUniformLocation(self.id, s.as_ptr());
            gl::UseProgram(self.id);
            gl::Uniform1ui(loc, value);
        }
    }
}
