//! This module abstracts an OpenGL SSBO.
//! It implements the drop trait for automatic clean-up.
use std::ffi::c_void;

use crate::gpu::GpuSsbo;

pub struct SSBO {
    id: u32,
}

impl Drop for SSBO {
    fn drop(&mut self) {
        unsafe { gl::DeleteBuffers(1, &self.id) }
    }
}

impl SSBO {
    /// Creates a new ssbo on the gpu and copies the objects data to it
    /// 
    /// # Arguments 
    /// * `binding` - Binding index. See https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindBufferBase.xhtml
    /// * `object` - The object to be moved to gpu memory. Needs to implement GpuSsbo.
    /// * `usage` - Memory usage pattern. See https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBufferData.xhtml
    /// 
    /// # Examples 
    /// ```
    /// let vec = vec![0 as u32; 10];
    /// let ssbo = SSBO::create_from(1, &vec, gl::STATIC_DRAW);
    /// ```
    pub fn create_from<T: GpuSsbo>(binding: u32, object: &T, usage: gl::types::GLenum) -> SSBO {
        let mut ssbo_id = 0 as u32;

        unsafe {
            gl::GenBuffers(1, &mut ssbo_id);

            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, ssbo_id);
            gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, binding, ssbo_id);

            let (data, len) = object.raw();
            gl::BufferData(gl::SHADER_STORAGE_BUFFER, len, data, usage);

            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, 0);
        }

        SSBO { id: ssbo_id }
    }

    /// Creates a new ssbo with a given size
    /// 
    /// # Arguments
    /// * `binding` - Binding index. See https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindBufferBase.xhtml
    /// * `size` - Size of the buffer.
    /// * `usage` - Memory usage pattern. See https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBufferData.xhtml
    /// 
    /// # Examples
    /// ```
    /// // We want to store [1 as u32; 10]
    /// // u32 corresponds to OpenGLs uint
    /// let ssbo = SSBO::create_empty(0, 10 * gl::types::GLuint, gl::STATIC_DRAW); 
    /// ```
    pub fn create_empty<T: GpuSsbo>(binding: u32, size: isize, usage: gl::types::GLenum) -> SSBO {
        let mut ssbo_id = 0 as u32;

        unsafe {
            gl::GenBuffers(1, &mut ssbo_id);

            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, ssbo_id);
            gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, binding, ssbo_id);

            gl::BufferData(gl::SHADER_STORAGE_BUFFER, size, std::ptr::null(), usage);

            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, 0);
        }

        SSBO { id: ssbo_id }
    }

    /// Updates the gpu memory of this buffer with the given data
    pub fn update<T: GpuSsbo>(&self, object: &T, offset: isize) {
        unsafe {
            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, self.id);

            let (data, len) = object.raw();
            gl::NamedBufferSubData(self.id, offset, len, data);

            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, 0);
        }
    }

    /// Moves data from the gpu to the main memory
    pub fn retrieve<T: GpuSsbo>(&self, object: &mut T, offset: isize) {
        unsafe {
            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, self.id);

            let (data, len) = object.raw();

            gl::GetNamedBufferSubData(self.id, offset, len, data as *mut c_void);

            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, 0);
        }
    }
}
