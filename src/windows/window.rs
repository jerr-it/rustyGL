use crate::{Color, PipelineShader};

use super::WindowBuilder;

pub struct Window {
    pub(super) window: sdl2::video::Window,
    pub(super) gl_context: sdl2::video::GLContext,
    pub(super) pipeline_shader: PipelineShader,
}

impl Window {
    pub fn new() -> WindowBuilder {
        WindowBuilder::default()
    }

    pub fn clear(&self, color: Color<f32>) {
        unsafe {
            gl::ClearColor(color.x, color.y, color.z, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn gl_swap(&self) {
        self.window.gl_swap_window();
    }
}
