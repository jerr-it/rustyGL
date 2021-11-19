//! This module abstracts an OpenGL SSBO.
//! It implements the drop trait for automatic clean-up.
use crate::gpu::GPU;

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
    pub fn create_from<T: GPU>(binding: u32, object: &T, usage: gl::types::GLenum) -> SSBO {
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

    /// Creates a new ssbo with the given length
    pub fn create_empty<T: GPU>(binding: u32, len: isize, usage: gl::types::GLenum) -> SSBO {
        let mut ssbo_id = 0 as u32;

        unsafe {
            gl::GenBuffers(1, &mut ssbo_id);

            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, ssbo_id);
            gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, binding, ssbo_id);

            gl::BufferData(gl::SHADER_STORAGE_BUFFER, len, std::ptr::null(), usage);

            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, 0);
        }

        SSBO { id: ssbo_id }
    }

    /// Updates the gpu memory of this buffer with the given data
    fn update<T: GPU>(&self, object: &T, offset: isize) {
        unsafe {
            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, self.id);

            let (data, len) = object.raw();
            gl::NamedBufferSubData(self.id, offset, len, data);

            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, 0);
        }
    }

    /// Moves data from the gpu to the main memory
    pub fn retrieve<T: Default>(&self, offset: isize) -> T {
        let data: T = Default::default();

        unsafe {
            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, self.id);

            gl::GetNamedBufferSubData(
                self.id,
                offset,
                std::mem::size_of::<T>() as isize,
                std::mem::transmute(&data),
            );

            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, 0);
        }

        data
    }
}
