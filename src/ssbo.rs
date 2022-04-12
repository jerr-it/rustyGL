//! This module abstracts an OpenGL SSBO.
//! It implements the drop trait for automatic clean-up.

use std::{
    ffi::c_void,
    ops::{Deref, DerefMut},
};

use crate::GPU;

pub struct SSBO<T> {
    id: u32,
    content: T,
}

impl<T> Drop for SSBO<T> {
    fn drop(&mut self) {
        unsafe { gl::DeleteBuffers(1, &self.id) }
    }
}

impl<T: GPU> Deref for SSBO<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.content
    }
}

impl<T: GPU> DerefMut for SSBO<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.content
    }
}

impl<T: GPU> SSBO<T> {
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
    /// let ssbo = SSBO::create_from(1, vec, gl::STATIC_DRAW);
    /// ```
    pub fn create_from(binding: u32, object: T, usage: gl::types::GLenum) -> SSBO<T> {
        let mut ssbo_id = 0 as u32;

        unsafe {
            gl::GenBuffers(1, &mut ssbo_id);

            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, ssbo_id);
            gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, binding, ssbo_id);

            let (data, len) = object.raw();
            gl::BufferData(gl::SHADER_STORAGE_BUFFER, len, data, usage);

            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, 0);
        }

        let mut ssbo = SSBO {
            id: ssbo_id,
            content: object,
        };

        ssbo.update();

        ssbo
    }

    /// Sends content to the gpu
    pub fn update(&mut self) {
        unsafe {
            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, self.id);

            let (data, len) = self.content.raw();

            gl::NamedBufferSubData(self.id, 0, len, data as *mut c_void);

            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, 0);
        }
    }

    /// Retrieves the data from the gpu and stores it back into content
    pub fn load(&mut self) {
        unsafe {
            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, self.id);

            let (data, len) = self.content.raw();

            gl::GetNamedBufferSubData(self.id, 0, len, data as *mut c_void);

            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, 0);
        }
    }
}
