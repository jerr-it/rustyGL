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
    /// Constructs a compute shader from a given file or string.
    /// 
    /// # Arguments
    /// * `source` - Source for the compute shader
    /// 
    /// # Examples
    /// ```
    /// const SHADER_SOURCE: &str = "
    ///     #version 430
    ///     layout(local_size_x = 1, local_size_y = 1) in;
    /// 
    ///     void main() {
    /// 
    ///     }
    /// ";
    /// 
    /// let shader = ComputeShader::create(ShaderSource::String(SHADER_SOURCE))?;
    /// ```
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
    /// 
    /// # Arguments
    /// * `num_groups_x` - Number of work groups to be launched in the x dimension
    /// * `num_groups_y` - Number of work groups to be launched in the y dimension
    /// * `num_groups_z` - Number of work groups to be launched in the z dimension. See https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDispatchCompute.xhtml for more information
    /// * `barrier` - The type of memory barrier to be used. See https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMemoryBarrier.xhtml
    /// 
    /// # Examples
    /// ```
    /// const SHADER_SOURCE: &str = "
    ///     #version 430
    ///     layout(local_size_x = 1, local_size_y = 1) in;
    /// 
    ///     void main() {
    /// 
    ///     }
    /// ";
    /// 
    /// let shader = ComputeShader::create(ShaderSource::String(SHADER_SOURCE))?;
    /// shader.dispatch(1,1,1, gl::SHADER_STORAGE_BARRIER_BIT);
    /// ```
    pub fn dispatch(&self, num_groups_x: u32, num_groups_y: u32, num_groups_z: u32, barrier: u32) {
        unsafe {
            gl::UseProgram(self.id);
            gl::DispatchCompute(num_groups_x, num_groups_y, num_groups_z);
            gl::MemoryBarrier(barrier);
        }
    }
}
