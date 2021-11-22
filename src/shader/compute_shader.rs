use super::{compile_shader, link_program, ShaderSource};

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

        let shader = unsafe { gl::CreateShader(gl::COMPUTE_SHADER) };
        compile_shader(&src, shader)?;

        let program_id = unsafe { gl::CreateProgram() };
        unsafe {
            gl::AttachShader(program_id, shader);
        };

        link_program(program_id)?;

        unsafe {
            gl::DeleteShader(shader);
        };

        Ok(ComputeShader { id: program_id })
    }

    /// Dispatch this compute shader
    pub fn dispatch(&self, num_groups_x: u32, num_groups_y: u32, num_groups_z: u32, barrier: u32) {
        unsafe {
            gl::UseProgram(self.id);
            gl::DispatchCompute(num_groups_x, num_groups_y, num_groups_z);
            gl::MemoryBarrier(barrier);
        }
    }
}
