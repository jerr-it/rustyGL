use std::ffi::c_void;

use crate::vector::Vector3;

pub struct VBO {
    id: u32,
}

impl Drop for VBO {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}

impl VBO {
    pub fn new(vertices: Option<&Vec<Vector3<f32>>>) -> VBO {
        let mut id = 0 as u32;
        unsafe {
            gl::GenBuffers(1, &mut id);
        }

        match vertices {
            Some(verts) => unsafe {
                gl::BindBuffer(gl::ARRAY_BUFFER, id);

                gl::BufferData(
                    gl::ARRAY_BUFFER,
                    verts.len() as isize * std::mem::size_of::<Vector3<f32>>() as isize,
                    verts.as_ptr() as *const _,
                    gl::STATIC_DRAW,
                );
            },
            None => {}
        }

        VBO { id }
    }

    pub fn set_attributes(
        &self,
        location: u32,
        size: i32,
        gl_type: gl::types::GLenum,
        normalized: gl::types::GLboolean,
        stride: i32,
        offset: *const c_void,
    ) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
            gl::VertexAttribPointer(location, size, gl_type, normalized, stride, offset);
            gl::EnableVertexAttribArray(location);
        }
    }

    pub fn transfer(&self, vertices: &Vec<Vector3<f32>>) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
            gl::BufferSubData(
                gl::ARRAY_BUFFER,
                0,
                vertices.len() as isize * std::mem::size_of::<Vector3<f32>>() as isize,
                vertices.as_ptr() as *const _,
            );
        }
    }
}
