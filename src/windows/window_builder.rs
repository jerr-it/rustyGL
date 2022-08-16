use sdl2::VideoSubsystem;

use crate::{PipelineShader, ShaderSource};

use super::Window;

const DEFAULT_VERTEX_SHADER_PATH: &str = "src/windows/shader/vertex_shader_default.glsl";
const DEFAULT_FRAGMENT_SHADER_PATH: &str = "src/windows/shader/fragment_shader_default.glsl";

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
            vertex_shader: ShaderSource::File(DEFAULT_VERTEX_SHADER_PATH),
            fragment_shader: ShaderSource::File(DEFAULT_FRAGMENT_SHADER_PATH),
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

        let gl_context = window.gl_create_context()?;

        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const _);

        Ok(Window {
            window,
            gl_context,
            pipeline_shader: PipelineShader::create(
                Some(self.vertex_shader),
                Some(self.fragment_shader),
            )?,
        })
    }
}
