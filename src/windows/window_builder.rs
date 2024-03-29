use sdl2::VideoSubsystem;
use vector::Vector2;

use crate::{PipelineShader, ShaderSource};

use super::{
    default_shaders::{FRAGMENT_SHADER, VERTEX_SHADER},
    Window,
};

pub struct WindowBuilder {
    pub(super) title: String,
    pub(super) width: u32,
    pub(super) height: u32,
    pub(super) fullscreen: bool,
    pub(super) resizable: bool,
    pub(super) vertex_shader: ShaderSource,
    pub(super) fragment_shader: ShaderSource,
}

impl Default for WindowBuilder {
    fn default() -> Self {
        Self {
            title: "Window".to_string(),
            width: 400,
            height: 400,
            fullscreen: false,
            resizable: false,
            vertex_shader: ShaderSource::String(VERTEX_SHADER),
            fragment_shader: ShaderSource::String(FRAGMENT_SHADER),
        }
    }
}

impl WindowBuilder {
    pub fn title(self, title: &str) -> WindowBuilder {
        WindowBuilder {
            title: title.to_string(),
            ..self
        }
    }

    pub fn dimensions(self, width: u32, height: u32) -> WindowBuilder {
        WindowBuilder {
            width,
            height,
            ..self
        }
    }

    pub fn fullscreen(self, fullscreen: bool) -> WindowBuilder {
        WindowBuilder { fullscreen, ..self }
    }

    pub fn resizable(self, resizable: bool) -> WindowBuilder {
        WindowBuilder { resizable, ..self }
    }

    pub fn vertex_shader(self, vertex_shader: ShaderSource) -> WindowBuilder {
        WindowBuilder {
            vertex_shader,
            ..self
        }
    }

    pub fn fragment_shader(self, fragment_shader: ShaderSource) -> WindowBuilder {
        WindowBuilder {
            fragment_shader,
            ..self
        }
    }

    pub fn build(
        self,
        video_subsystem: &VideoSubsystem,
    ) -> Result<Window, Box<dyn std::error::Error>> {
        let mut window = video_subsystem.window(self.title.as_str(), self.width, self.height);

        if self.fullscreen {
            window.fullscreen();
        }

        if self.resizable {
            window.resizable();
        }

        window.opengl();

        let window = window.build()?;

        let _gl_context = window.gl_create_context()?;

        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const _);

        let pipeline_shader =
            PipelineShader::create(Some(self.vertex_shader), Some(self.fragment_shader))?;
        pipeline_shader.enable();

        pipeline_shader.set_uniform("resolution", Vector2::new(self.width, self.height));

        Ok(Window {
            window,
            _gl_context,
            pipeline_shader,
        })
    }
}
