//! This module implements a trait that should be implemented
//! by types which are to be transferred to the gpu.
//! A default implementation and some special types are implemented already

use std::ffi::c_void;

/// Should be implemented by types which need to be transferred to the gpu
pub trait GPU {
    /// Returns a pointer and the corresponding datas length
    fn raw(&self) -> (*const c_void, isize) {
        let len = std::mem::size_of_val(self) as isize;
        (self as *const _ as *const c_void, len)
    }
}

/// Implements the gpu trait for generic vectors
impl<T> GPU for Vec<T> {
    fn raw(&self) -> (*const c_void, isize) {
        let len = (std::mem::size_of::<T>() * self.len()) as isize;
        (self.as_ptr() as *const c_void, len)
    }
}

/// Implements the gpu trait for a touple
impl<T, U> GPU for (T, U) {
    fn raw(&self) -> (*const c_void, isize) {
        let len = (std::mem::size_of::<T>() + std::mem::size_of::<U>()) as isize;
        (self as *const _ as *const c_void, len)
    }
}
