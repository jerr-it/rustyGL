use std::ffi::c_void;

use crate::{vertices::{Vertex, VBO, VAO, EBO}, vector::{Vector2, Vector3}, Color};

use super::shape::{Shape2D, Drawable};

pub struct Rectangle {
    center: Vector2<f32>,
    vertices: Vec<Vertex>,

    vao: VAO,
    vbo: VBO,
    ebo: EBO
}

impl Drawable for Rectangle {
    fn draw(&self) {
        self.vao.draw(gl::TRIANGLES, self.ebo.len() as i32, true);
    }
}

impl Shape2D for Rectangle {
    fn translate(&mut self, translation: Vector2<f32>) {
        self.center += translation;
    }

    fn rotate(&mut self, angle: f32) {
        for vertex in self.vertices.iter_mut() {
            let position = vertex.position_mut();
            
            let (x, y, _) = position.components_mut();

            *x = *x * angle.cos() - *y * angle.sin();
            *y = *x * angle.sin() + *y * angle.cos();
        }
    }

    fn scale(&mut self, scl: f32) {
        for vertex in self.vertices.iter_mut() {
            *vertex.position_mut() *= scl;
        }
    }
}

impl Rectangle {
    /// Creates a new rectangle
    /// 
    /// # Arguments
    /// * `center` - Center point of the rectangle
    /// * `dimensions` - Horizontal and vertical size of the rectangle
    /// * `colors` - Color for each corner. If only one color is specified it will be used for all corners. White will be used if no color is specified.
    /// ```
    pub fn new(center: Vector2<f32>, dimensions: Vector2<f32>, colors: Option<Vec<Color<f32>>>) -> Rectangle {
        let (x, y) = center.components();
        let (width, height) = dimensions.components();

        let half_width = *width/2.0;
        let half_height = *height/2.0;

        let corners = vec![
            Vector2::<f32>::new(*x - half_width, *y - half_height),
            Vector2::<f32>::new(*x + half_width, *y - half_height),
            Vector2::<f32>::new(*x + half_width, *y + half_height),
            Vector2::<f32>::new(*x - half_width, *y + half_height),
        ];

        let colors = match colors {
            Some(vec) => match vec.len() {
                1 => Vec::from([vec[0]; 4]),
                4 => vec,
                _ => Vec::from([Color::<f32>::new(1.0, 1.0, 1.0); 4])
            },
            None => Vec::from([Color::<f32>::new(1.0, 1.0, 1.0); 4]),
        };

        let mut color_iter = colors.iter();
        let vertices = corners.iter().map(|corner| {
            let color = color_iter.next().unwrap();
            Vertex::new((*corner).as_vector3(), *color, Vector2::<f32>::new(0.0,0.0))
        }).collect();

        let indices = vec![0, 1, 2, 0, 2, 3];

        let vao = VAO::new();
        let vbo = VBO::new(Some(&vertices));

        vbo.set_attributes(
            0, 
            3, 
            gl::FLOAT, 
            gl::FALSE, 
            std::mem::size_of::<Vertex>() as i32, 
            std::ptr::null(),
        );

        vbo.set_attributes(
            1, 
            3, 
            gl::FLOAT, 
            gl::FALSE, 
            std::mem::size_of::<Vertex>() as i32, 
            unsafe { std::ptr::null::<Vector3<f32>>().add(1) as *const c_void },
        );

        vbo.set_attributes(
            2, 
            2, 
            gl::FLOAT, 
            gl::FALSE, 
            std::mem::size_of::<Vertex>() as i32, 
            unsafe { std::ptr::null::<Vector3<f32>>().add(2) as *const c_void }
        );

        let ebo = EBO::new(Some(&indices));

        Rectangle {
            center,
            vertices,
            vao, 
            vbo,
            ebo
        }
    }
}