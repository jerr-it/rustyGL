pub struct VAO {
    id: u32,
}

impl Drop for VAO {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.id);
        }
    }
}

impl VAO {
    /// Creates a new VAO, which bound immediately using gl::BindVertexArray
    pub fn new() -> VAO {
        let mut id = 0 as u32;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
            gl::BindVertexArray(id);
        }

        VAO { id }
    }

    pub fn draw(&self, mode: u32, vertex_count: i32, has_ebo: bool) {
        unsafe {
            gl::BindVertexArray(self.id);
            if has_ebo {
                gl::DrawElements(mode, vertex_count, gl::UNSIGNED_INT, std::ptr::null());
            } else {
                gl::DrawArrays(mode, 0, vertex_count);
            }
        }
    }
}
