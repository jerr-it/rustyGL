use super::{compile_shader, link_program, string_from_shader_source, ShaderSource};

pub struct PipelineShader {
    id: u32,
}

impl Drop for PipelineShader {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.id) }
    }
}

impl PipelineShader {
    pub fn create(
        vertex_shader_source: Option<ShaderSource>,
        fragment_shader_source: Option<ShaderSource>,
    ) -> Result<PipelineShader, Box<dyn std::error::Error>> {
        let vertex_source = string_from_shader_source(vertex_shader_source)?;
        let fragment_source = string_from_shader_source(fragment_shader_source)?;

        let vertex_shader = unsafe { gl::CreateShader(gl::VERTEX_SHADER) };
        let fragment_shader = unsafe { gl::CreateShader(gl::FRAGMENT_SHADER) };

        let program_id = unsafe { gl::CreateProgram() };

        match &vertex_source {
            Some(source) => {
                compile_shader(source, vertex_shader)?;
                unsafe {
                    gl::AttachShader(program_id, vertex_shader);
                };
            }
            None => {}
        }

        match &fragment_source {
            Some(source) => {
                compile_shader(source, fragment_shader)?;
                unsafe {
                    gl::AttachShader(program_id, fragment_shader);
                };
            }
            None => {}
        }

        if let Err(e) = link_program(program_id) {
            Err(e)?
        }

        if vertex_source.is_some() {
            unsafe {
                gl::DeleteShader(vertex_shader);
            };
        }

        if fragment_source.is_some() {
            unsafe {
                gl::DeleteShader(fragment_shader);
            };
        }

        Ok(PipelineShader { id: program_id })
    }
}
