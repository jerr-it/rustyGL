use crate::PipelineShader;

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

    pub fn gl_swap(&self) {
        self.window.gl_swap_window();
    }
}
