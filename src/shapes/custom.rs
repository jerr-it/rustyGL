use std::ffi::c_void;

use vector::{Vector2, Vector3};

use crate::{
    vertices::{Vertex, VAO, VBO},
    PipelineShader,
};

use super::{Drawable, Shape2D};

pub struct CustomShape2D {
    center: Vector2<f32>,
    vertices: Vec<Vertex>, // position is relative to center

    draw_mode: gl::types::GLenum,

    vao: VAO,
    vbo: VBO,
}

impl Drawable for CustomShape2D {
    fn draw(&self, shader: &PipelineShader) {
        let v3: Vector3<f32> = self.center.into();
        shader.set_uniform("center", v3);
        self.vao
            .draw(self.draw_mode, self.vertices.len() as i32, false);
    }
}

impl Shape2D for CustomShape2D {
    fn translate(&mut self, translation: Vector2<f32>) -> &mut Self {
        self.center += translation;
        self
    }

    fn rotate(&mut self, angle: f32) -> &mut Self {
        todo!()
    }

    fn scale(&mut self, scl: f32) -> &mut Self {
        todo!()
    }
}

impl CustomShape2D {
    /// Create a new custom shape.
    /// # Arguments
    /// * `vertices` - The vertices of the shape. If center is given, the vertices are relative to the center. Otherwise, they are relative to the origin and the center will be the average of the vertices.
    /// * `center` - Center of the shape.
    /// * `draw_mode` - The draw mode to use (POINTS, LINES, LINE_STRIP ...).
    ///
    /// Averages the vertices to determine the center
    ///
    /// # Example
    /// ```
    /// const VERT_SHADER: &str = "
    ///     #version 430
    ///     layout (location = 4) in vec3 vPos;
    ///     layout (location = 5) in vec3 vColor;
    ///     layout (location = 6) in vec2 vTexCoord;
    ///     out vec3 outColor;
    ///     void main() {
    ///         gl_Position = vec4(vPos.x, vPos.y, vPos.z, 1.0);
    ///         outColor = vColor;
    ///     }
    /// ";
    ///
    /// let custom_shape_points = CustomShape::new(vertices, gl::TRIANGLE_FAN)
    ///     .position_shader_location(4)
    ///     .color_shader_location(5)
    ///     .uv_shader_location(6);
    /// ```
    pub fn new(vertices: Vec<Vertex>, draw_mode: gl::types::GLenum) -> CustomShape2D {
        // Calculate center by averaging the vertices
        let center = vertices
            .iter()
            .fold(Vector2::default(), |acc, v| acc + (*v.position()).into());
        let center = center * (1.0 / vertices.len() as f32);

        // Translate the vertices to the center
        let vertices = vertices
            .into_iter()
            .map(|v| {
                let pos = v.position();
                let pos = *pos - center.into();
                Vertex::new(pos.into(), *v.color(), *v.texture_coord())
            })
            .collect();

        let vao = VAO::new();
        let vbo = VBO::new(Some(&vertices));

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

        CustomShape2D {
            center,
            vertices,
            draw_mode,
            vao,
            vbo,
        }
    }

    /// Optionally set the position shader location of the shape.
    /// # Arguments
    /// * `location` - The location of the position shader attribute.
    ///
    /// # Example
    /// ```
    /// const VERT_SHADER: &str = "
    ///     #version 430
    ///     layout (location = 4) in vec3 vPos;
    ///     layout (location = 5) in vec3 vColor;
    ///     layout (location = 6) in vec2 vTexCoord;
    ///     out vec3 outColor;
    ///     void main() {
    ///         gl_Position = vec4(vPos.x, vPos.y, vPos.z, 1.0);
    ///         outColor = vColor;
    ///     }
    /// ";
    ///
    /// let custom_shape_points = CustomShape::new(vertices, gl::TRIANGLE_FAN)
    ///     .position_shader_location(4)
    ///     .color_shader_location(5)
    ///     .uv_shader_location(6);
    /// ```
    pub fn position_shader_location(self, location: u32) -> Self {
        self.vbo.set_attributes(
            location,
            3,
            gl::FLOAT,
            gl::FALSE,
            std::mem::size_of::<Vertex>() as i32,
            std::ptr::null(),
        );

        self
    }

    /// Optionally set the color shader location of the shape.
    /// # Arguments
    /// * `location` - The location of the position shader attribute.
    ///
    /// # Example
    /// ```
    /// const VERT_SHADER: &str = "
    ///     #version 430
    ///     layout (location = 4) in vec3 vPos;
    ///     layout (location = 5) in vec3 vColor;
    ///     layout (location = 6) in vec2 vTexCoord;
    ///     out vec3 outColor;
    ///     void main() {
    ///         gl_Position = vec4(vPos.x, vPos.y, vPos.z, 1.0);
    ///         outColor = vColor;
    ///     }
    /// ";
    ///
    /// let custom_shape_points = CustomShape::new(vertices, gl::TRIANGLE_FAN)
    ///     .position_shader_location(4)
    ///     .color_shader_location(5)
    ///     .uv_shader_location(6);
    /// ```
    pub fn color_shader_location(self, location: u32) -> Self {
        self.vbo.set_attributes(
            location,
            3,
            gl::FLOAT,
            gl::FALSE,
            std::mem::size_of::<Vertex>() as i32,
            unsafe { std::ptr::null::<Vector3<f32>>().add(1) as *const c_void },
        );

        self
    }

    /// Optionally set the uv shader location of the shape.
    /// # Arguments
    /// * `location` - The location of the position shader attribute.
    ///
    /// # Example
    /// ```
    /// const VERT_SHADER: &str = "
    ///     #version 430
    ///     layout (location = 4) in vec3 vPos;
    ///     layout (location = 5) in vec3 vColor;
    ///     layout (location = 6) in vec2 vTexCoord;
    ///     out vec3 outColor;
    ///     void main() {
    ///         gl_Position = vec4(vPos.x, vPos.y, vPos.z, 1.0);
    ///         outColor = vColor;
    ///     }
    /// ";
    ///
    /// let custom_shape_points = CustomShape::new(vertices, gl::TRIANGLE_FAN)
    ///     .position_shader_location(4)
    ///     .color_shader_location(5)
    ///     .uv_shader_location(6);
    /// ```
    pub fn uv_shader_location(self, location: u32) -> Self {
        self.vbo.set_attributes(
            location,
            2,
            gl::FLOAT,
            gl::FALSE,
            std::mem::size_of::<Vertex>() as i32,
            unsafe { std::ptr::null::<Vector3<f32>>().add(2) as *const c_void },
        );

        self
    }

    /// Specify a custom center location for the shape.
    /// # Arguments
    /// * `center` - The center location of the shape.
    /// # Example
    /// ```
    /// const VERT_SHADER: &str = "
    ///    #version 430
    ///    layout (location = 4) in vec3 vPos;
    ///    layout (location = 5) in vec3 vColor;
    ///    layout (location = 6) in vec2 vTexCoord;
    ///    out vec3 outColor;
    ///    void main() {
    ///        gl_Position = vec4(vPos.x, vPos.y, vPos.z, 1.0);
    ///        outColor = vColor;
    ///    }
    /// ";
    /// let custom_shape_points = CustomShape::new(vertices, gl::TRIANGLE_FAN)
    ///    .center(Vector2::new(20.0, 20.0))
    ///    .position_shader_location(4)
    ///    .color_shader_location(5)
    ///    .uv_shader_location(6);
    /// ```
    pub fn center(mut self, center: Vector2<f32>) -> Self {
        self.center = center;
        self
    }
}
