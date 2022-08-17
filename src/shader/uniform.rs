use std::ffi::CString;

use vector::{Vector2, Vector3, Vector4};

pub trait Uniform {
    fn transfer(&self, shader_id: u32, name: CString);
}

impl Uniform for i32 {
    fn transfer(&self, shader_id: u32, name: CString) {
        unsafe {
            gl::Uniform1i(
                gl::GetUniformLocation(shader_id, name.as_ptr() as *const i8),
                *self,
            );
        }
    }
}

impl Uniform for Vector2<i32> {
    fn transfer(&self, shader_id: u32, name: CString) {
        unsafe {
            gl::Uniform2i(
                gl::GetUniformLocation(shader_id, name.as_ptr() as *const i8),
                self.x,
                self.y,
            );
        }
    }
}

impl Uniform for Vector3<i32> {
    fn transfer(&self, shader_id: u32, name: CString) {
        unsafe {
            gl::Uniform3i(
                gl::GetUniformLocation(shader_id, name.as_ptr() as *const i8),
                self.x,
                self.y,
                self.z,
            );
        }
    }
}

impl Uniform for Vector4<i32> {
    fn transfer(&self, shader_id: u32, name: CString) {
        unsafe {
            gl::Uniform4i(
                gl::GetUniformLocation(shader_id, name.as_ptr() as *const i8),
                self.x,
                self.y,
                self.z,
                self.w,
            );
        }
    }
}

// TODO implement other types of uniforms
// maybe use a macro to generate the transfer function?
