use std::ffi::c_void;

use vector::Vector3;

use crate::vertices::{Vertex, VAO, VBO};

use super::Drawable;

pub struct CustomShape {
    vertices: Vec<Vertex>,

    draw_mode: gl::types::GLenum,

    vao: VAO,
    _vbo: VBO,
}

impl Drawable for CustomShape {
    fn draw(&self) {
        self.vao
            .draw(self.draw_mode, self.vertices.len() as i32, false);
    }
}

impl CustomShape {
    pub fn new(
        vertices: Vec<Vertex>,
        draw_mode: gl::types::GLenum,
        position_location: Option<u32>,
        color_location: Option<u32>,
        uv_location: Option<u32>,
    ) -> CustomShape {
        let vao = VAO::new();
        let vbo = VBO::new(Some(&vertices));

        vbo.set_attributes(
            match position_location {
                Some(location) => location,
                None => 0,
            },
            3,
            gl::FLOAT,
            gl::FALSE,
            std::mem::size_of::<Vertex>() as i32,
            std::ptr::null(),
        );

        vbo.set_attributes(
            match color_location {
                Some(location) => location,
                None => 1,
            },
            3,
            gl::FLOAT,
            gl::FALSE,
            std::mem::size_of::<Vertex>() as i32,
            unsafe { std::ptr::null::<Vector3<f32>>().add(1) as *const c_void },
        );

        vbo.set_attributes(
            match uv_location {
                Some(location) => location,
                None => 2,
            },
            2,
            gl::FLOAT,
            gl::FALSE,
            std::mem::size_of::<Vertex>() as i32,
            unsafe { std::ptr::null::<Vector3<f32>>().add(2) as *const c_void },
        );

        CustomShape {
            vertices,
            draw_mode,
            vao,
            _vbo: vbo,
        }
    }
}
