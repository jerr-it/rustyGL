use std::ffi::CString;

use crate::vector::{Vector2, Vector3, Vector4};

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
    /// Creates a new PipelineShader.
    /// Must contain either a vertex shader or a fragment shader or both.
    ///
    /// # Arguments
    /// * `vertex_shader_source` - Source for the vertex shader
    /// * `fragment_shader_source` - Source for the fragment shader
    ///
    /// # Examples
    /// ```
    /// const FRAG_SHADER: &str = "
    ///     #version 330 core
    ///     out vec4 FragColor;
    ///
    ///     void main()
    ///     {
    ///         FragColor = vec4(1.0, 1.0, 1.0, 1.0);
    ///     }
    /// ";
    ///
    /// let shader = PipelineShader::create(Some(ShaderSource::String(FRAG_SHADER)), None)?;
    /// ```
    pub fn create(
        vertex_shader_source: Option<ShaderSource>,
        fragment_shader_source: Option<ShaderSource>,
    ) -> Result<PipelineShader, Box<dyn std::error::Error>> {
        if *(&vertex_shader_source.is_none()) && *(&fragment_shader_source.is_none()) {
            Err("No shader input given")?
        }

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

    /// Use this pipeline shaders.
    ///
    /// # Examples
    /// ```
    /// const FRAG_SHADER: &str = "
    ///     #version 430
    ///     layout(local_size_x = 1, local_size_y = 1) in;
    ///
    ///     void main() {}";
    ///
    /// let shader = PipelineShader::create(Some(ShaderSource::String(FRAG_SHADER)), None)?;
    /// shader.enable();
    /// ```
    pub fn enable(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn set_uniform_bool(&self, name: &str, value: bool) {
        unsafe {
            gl::UseProgram(self.id);
            let name = CString::new(name).unwrap();

            gl::Uniform1i(
                gl::GetUniformLocation(self.id, name.as_ptr() as *const i8), 
                value as i32
            );
        }
    }

    pub fn set_uniform_bool2(&self, name: &str, value: Vector2<bool>) {
        unsafe {
            gl::UseProgram(self.id);
            let name = CString::new(name).unwrap();

            gl::Uniform2i(
                gl::GetUniformLocation(self.id, name.as_ptr() as *const i8), 
                value.x() as i32, 
                value.y() as i32
            );
        }
    }
    
    pub fn set_uniform_bool3(&self, name: &str, value: Vector3<bool>) {
        unsafe {
            gl::UseProgram(self.id);
            let name = CString::new(name).unwrap();

            gl::Uniform3i(
                gl::GetUniformLocation(self.id, name.as_ptr() as *const i8), 
                value.x() as i32, 
                value.y() as i32, 
                value.z() as i32
            );
        }
    }

    pub fn set_uniform_bool4(&self, name: &str, value: Vector4<bool>) {
        unsafe {
            gl::UseProgram(self.id);
            let name = CString::new(name).unwrap();

            gl::Uniform4i(
                gl::GetUniformLocation(self.id, name.as_ptr() as *const i8), 
                value.x() as i32, 
                value.y() as i32, 
                value.z() as i32, 
                value.w() as i32
            );
        }
    }

    pub fn set_uniform_float(&self, name: &str, value: f32) {
        unsafe {
            gl::UseProgram(self.id);
            let name = CString::new(name).unwrap();

            gl::Uniform1f(
                gl::GetUniformLocation(self.id, name.as_ptr() as *const i8), 
                value
            );
        }
    }

    pub fn set_uniform_float2(&self, name: &str, value: Vector2<f32>) {
        unsafe {
            gl::UseProgram(self.id);
            let name = CString::new(name).unwrap();

            gl::Uniform2f(
                gl::GetUniformLocation(self.id, name.as_ptr() as *const i8), 
                value.x(), 
                value.y()
            );
        }
    }
    
    pub fn set_uniform_float3(&self, name: &str, value: Vector3<f32>) {
        unsafe {
            gl::UseProgram(self.id);
            let name = CString::new(name).unwrap();

            gl::Uniform3f(
                gl::GetUniformLocation(self.id, name.as_ptr() as *const i8), 
                value.x(), 
                value.y(), 
                value.z()
            );
        }
    }

    pub fn set_uniform_float4(&self, name: &str, value: Vector4<f32>) {
        unsafe {
            gl::UseProgram(self.id);
            let name = CString::new(name).unwrap();

            gl::Uniform4f(
                gl::GetUniformLocation(self.id, name.as_ptr() as *const i8), 
                value.x(), 
                value.y(), 
                value.z(), 
                value.w()
            );
        }
    }

    pub fn set_uniform_int(&self, name: &str, value: i32) {
        unsafe {
            gl::UseProgram(self.id);
            let cstr = CString::new(name).unwrap();

            gl::Uniform1i(
                gl::GetUniformLocation(self.id, cstr.as_ptr() as *const i8), 
                value
            );
        }
    }

    pub fn set_uniform_int2(&self, name: &str, value: Vector2<i32>) {
        unsafe {
            gl::UseProgram(self.id);
            let name = CString::new(name).unwrap();

            gl::Uniform2i(
                gl::GetUniformLocation(self.id, name.as_ptr() as *const i8), 
                value.x(), 
                value.y()
            );
        }
    }
    
    pub fn set_uniform_int3(&self, name: &str, value: Vector3<i32>) {
        unsafe {
            gl::UseProgram(self.id);
            let name = CString::new(name).unwrap();

            gl::Uniform3i(
                gl::GetUniformLocation(self.id, name.as_ptr() as *const i8), 
                value.x(), 
                value.y(), 
                value.z()
            );
        }
    }

    pub fn set_uniform_int4(&self, name: &str, value: Vector4<i32>) {
        unsafe {
            gl::UseProgram(self.id);
            let name = CString::new(name).unwrap();

            gl::Uniform4i(
                gl::GetUniformLocation(self.id, name.as_ptr() as *const i8), 
                value.x(), 
                value.y(), 
                value.z(), 
                value.w()
            );
        }
    }

    pub fn set_uniform_uint(&self, name: &str, value: u32) {
        unsafe {
            gl::UseProgram(self.id);
            let name = CString::new(name).unwrap();

            gl::Uniform1ui(
                gl::GetUniformLocation(self.id, name.as_ptr() as *const i8), 
                value
            );
        }
    }

    pub fn set_uniform_uint2(&self, name: &str, value: Vector2<u32>) {
        unsafe {
            gl::UseProgram(self.id);
            let name = CString::new(name).unwrap();

            gl::Uniform2ui(
                gl::GetUniformLocation(self.id, name.as_ptr() as *const i8), 
                value.x(), 
                value.y()
            );
        }
    }
    
    pub fn set_uniform_uint3(&self, name: &str, value: Vector3<u32>) {
        unsafe {
            gl::UseProgram(self.id);
            let name = CString::new(name).unwrap();

            gl::Uniform3ui(
                gl::GetUniformLocation(self.id, name.as_ptr() as *const i8), 
                value.x(), 
                value.y(), 
                value.z()
            );
        }
    }

    pub fn set_uniform_uint4(&self, name: &str, value: Vector4<u32>) {
        unsafe {
            gl::UseProgram(self.id);
            let name = CString::new(name).unwrap();

            gl::Uniform4ui(
                gl::GetUniformLocation(self.id, name.as_ptr() as *const i8), 
                value.x(), 
                value.y(), 
                value.z(), 
                value.w()
            );
        }
    }
}
