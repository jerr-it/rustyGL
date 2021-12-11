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
    pub fn new(vertices: Option<&Vec<Vector3>>, location: u32) -> VBO {
        let mut id = 0 as u32;
        unsafe {
            gl::GenBuffers(1, &mut id);
        }

        match vertices {
            Some(verts) => unsafe {
                gl::BindBuffer(gl::ARRAY_BUFFER, id);

                gl::BufferData(
                    gl::ARRAY_BUFFER,
                    verts.len() as isize * std::mem::size_of::<Vector3>() as isize,
                    verts.as_ptr() as *const _,
                    gl::STATIC_DRAW,
                );
                gl::VertexAttribPointer(
                    location,
                    3,
                    gl::FLOAT,
                    gl::FALSE,
                    std::mem::size_of::<Vector3>() as i32,
                    std::ptr::null(),
                );
                gl::EnableVertexAttribArray(location);

                gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            },
            None => {}
        }

        VBO { id }
    }

    pub fn transfer(&self, vertices: &Vec<Vector3>) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
            gl::BufferSubData(
                gl::ARRAY_BUFFER,
                0,
                vertices.len() as isize * 3,
                vertices.as_ptr() as *const _,
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }
}
