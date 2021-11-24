//! This module implements a trait that should be implemented
//! by types which are to be transferred to the gpu.
//! A default implementation and some special types are implemented already

use std::ffi::c_void;
use crate::SSBO;

/// Should be implemented by types which need to be transferred to a ssbo
pub trait GpuSsbo {
    /// Returns a pointer and the corresponding datas length
    fn raw(&self) -> (*const c_void, isize) {
        let len = std::mem::size_of_val(self) as isize;
        (self as *const _ as *const c_void, len)
    }

    /// Saves this object to a given SSBO.
    /// Convenience function.
    /// 
    /// # Arguments
    /// * `ssbo` - The ssbo to save to
    /// * `offset` - Offset within the ssbo
    /// 
    /// # Examples
    /// ```
    /// let vec1 = [1 as u32; 10];
    /// let ssbo = SSBO::create_from(1, &vec1, gl::STATIC_DRAW);
    /// //SSBO content: [1, 1, ... , 1]
    /// 
    /// let vec2 = [2 as u32; 10];
    /// vec2.save_to(&ssbo, 0);
    /// //SSBO content: [2, 2, ... , 2]
    /// ```
    fn save_to(&self, ssbo: &SSBO, offset: isize)
    where
        Self: Sized,
    {
        ssbo.update(self, offset);
    }

    /// Loads the content of a given SSBO into self.
    /// Convenience function.
    /// 
    /// # Arguments
    /// * `ssbo` - The ssbo to load from
    /// * `offset` - The offset within the ssbo
    /// 
    /// # Examples
    /// ```
    /// let vec1 = [1 as u32; 10];
    /// let ssbo = SSBO::create_from(1, &vec1, gl::STATIC_DRAW);
    /// 
    /// let vec2 = [2 as u32; 10];
    /// vec2.load_from(&ssbo, 0);
    /// //vec2: [1, 1, ... , 1]
    /// ```
    fn load_from(&mut self, ssbo: &SSBO, offset: isize)
    where
        Self: Sized,
    {
        ssbo.retrieve(self, offset);
    }

    /// Creates a new SSBO for this object.
    /// Convenience function.
    /// 
    /// # Arguments
    /// * `binding` - Binding index. See https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindBufferBase.xhtml
    /// * `usage` - Usage hint. See https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBufferData.xhtml
    /// 
    /// # Examples
    /// ```
    /// let vec = [1 as u32; 10];
    /// let ssbo = vec.create_ssbo(1, gl::STATIC_DRAW);
    /// ```
    fn create_ssbo(&self, binding: u32, usage: gl::types::GLenum) -> SSBO where Self: Sized {
        SSBO::create_from(binding, self, usage)
    }
}

/// Implements the gpu trait for generic vectors
impl<T> GpuSsbo for Vec<T> {
    fn raw(&self) -> (*const c_void, isize) {
        let len = (std::mem::size_of::<T>() * self.len()) as isize;
        (self.as_ptr() as *const c_void, len)
    }
}

/// Implements the gpu trait for a touple
impl<T, U> GpuSsbo for (T, U) {
    fn raw(&self) -> (*const c_void, isize) {
        let len = (std::mem::size_of::<T>() + std::mem::size_of::<U>()) as isize;
        (self as *const _ as *const c_void, len)
    }
}
