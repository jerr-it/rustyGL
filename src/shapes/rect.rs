use std::ffi::c_void;

use vector::{Vector2, Vector3};

use crate::{
    vertices::{Vertex, EBO, VAO, VBO},
    PipelineShader,
};

use super::{Drawable, Shape2D};

pub struct Rect {
    center: Vector2<f32>,
    angle: f32,
    scale: f32,

    vertices: Vec<Vertex>,

    vao: VAO,
    vbo: VBO,
    ebo: EBO,
}

impl Drawable for Rect {
    fn draw(&self, shader: &PipelineShader) {
        let v3: Vector3<f32> = self.center.into();
        shader.set_uniform("center", v3);

        shader.set_uniform("angle", self.angle);

        shader.set_uniform("scale", self.scale);

        self.vao.draw(gl::TRIANGLE_STRIP, 6, true);
    }

    fn center_mut(&mut self) -> &mut Vector2<f32> {
        &mut self.center
    }

    fn angle_mut(&mut self) -> &mut f32 {
        &mut self.angle
    }

    fn scale_mut(&mut self) -> &mut f32 {
        &mut self.scale
    }

    fn vbo(&self) -> &VBO {
        &self.vbo
    }
}

impl Shape2D for Rect {}

impl Rect {
    /// Create a new rectangle shape.
    /// # Arguments
    /// * `vertices` - The vertices of the shape. Corners should be in clockwise order starting from the top left corner<br/>
    /// <pre>
    /// 0 -- 1
    /// |    |
    /// 3 -- 2
    /// <pre/>
    pub fn new(vertices: Vec<Vertex>) -> Rect {
        // Calculate the center of the shape by taking the average of the vertices
        let mut center = vertices.iter().fold(Vector2::new(0.0, 0.0), |acc, vertex| {
            acc + (*vertex.position()).into()
        });
        center *= 1.0 / vertices.len() as f32;

        // Adjust the vertices to be relative to the center
        let vertices = vertices
            .into_iter()
            .map(|vertex| {
                let position = vertex.position();
                Vertex::new(
                    Vector3::new(position.x - center.x, position.y - center.y, position.z),
                    *vertex.color(),
                    *vertex.texture_coord(),
                )
            })
            .collect::<Vec<Vertex>>();

        let vao = VAO::new();
        let vbo = VBO::new(Some(&vertices));
        //  0     0 - 1
        //  | \     \ |
        //  3 - 2     2
        let ebo = EBO::new(Some(&vec![0, 1, 2, 1, 2, 3]));

        vbo.set_attributes(
            0, // Default shader position location
            3,
            gl::FLOAT,
            gl::FALSE,
            std::mem::size_of::<Vertex>() as i32,
            std::ptr::null(),
        );

        vbo.set_attributes(
            1, // Default shader color location
            3,
            gl::FLOAT,
            gl::FALSE,
            std::mem::size_of::<Vertex>() as i32,
            unsafe { std::ptr::null::<Vector3<f32>>().add(1) as *const c_void },
        );

        vbo.set_attributes(
            2, // Default shader uv location
            2,
            gl::FLOAT,
            gl::FALSE,
            std::mem::size_of::<Vertex>() as i32,
            unsafe { std::ptr::null::<Vector3<f32>>().add(2) as *const c_void },
        );

        Rect {
            center,
            angle: 0.0,
            scale: 1.0,
            vertices,
            vao,
            vbo,
            ebo,
        }
    }
}
