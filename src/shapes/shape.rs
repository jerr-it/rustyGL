use std::ffi::c_void;

use vector::{Vector2, Vector3};

use crate::{
    vertices::{Vertex, VBO},
    PipelineShader,
};

pub trait Drawable {
    fn draw(&self, shader: &PipelineShader);

    fn center_mut(&mut self) -> &mut Vector2<f32>;
    fn angle_mut(&mut self) -> &mut f32;
    fn scale_mut(&mut self) -> &mut f32;

    fn vbo(&self) -> &VBO;
}

pub trait Shape2D: Drawable {
    /// Moves the shape in the given x, y direction
    fn translate(&mut self, translation: Vector2<f32>) -> &mut Self {
        *self.center_mut() += translation;
        self
    }

    /// Rotates the shape around its center by the given angle
    fn rotate(&mut self, angle: f32) -> &mut Self {
        *self.angle_mut() += angle;
        self
    }

    /// Scales the shape by the given factor
    fn scale(&mut self, scl: f32) -> &mut Self {
        *self.scale_mut() += scl;
        self
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
    fn position_shader_location(&mut self, location: u32) -> &mut Self {
        self.vbo().set_attributes(
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
    fn color_shader_location(&mut self, location: u32) -> &mut Self {
        self.vbo().set_attributes(
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
    fn uv_shader_location(&mut self, location: u32) -> &mut Self {
        self.vbo().set_attributes(
            location,
            2,
            gl::FLOAT,
            gl::FALSE,
            std::mem::size_of::<Vertex>() as i32,
            unsafe { std::ptr::null::<Vector3<f32>>().add(2) as *const c_void },
        );

        self
    }
}
