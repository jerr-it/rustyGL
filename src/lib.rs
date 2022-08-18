pub mod debug;
mod gpu;
mod shader;
pub mod shapes;
mod ssbo;
pub mod vertices;
mod windows;

pub use color::Color;
pub use gpu::GPU;
pub use shader::{ComputeShader, PipelineShader, ShaderSource};
pub use ssbo::SSBO;
pub use windows::Window;

pub mod color {
    pub type Color<T> = vector::Vector3<T>;

    pub const WHITE: Color<f32> = Color::new(1.0, 1.0, 1.0);
    pub const BLACK: Color<f32> = Color::new(0.0, 0.0, 0.0);
    pub const RED: Color<f32> = Color::new(1.0, 0.0, 0.0);
    pub const GREEN: Color<f32> = Color::new(0.0, 1.0, 0.0);
    pub const BLUE: Color<f32> = Color::new(0.0, 0.0, 1.0);
    pub const YELLOW: Color<f32> = Color::new(1.0, 1.0, 0.0);
    pub const CYAN: Color<f32> = Color::new(0.0, 1.0, 1.0);
    pub const MAGENTA: Color<f32> = Color::new(1.0, 0.0, 1.0);
    pub const PURPLE: Color<f32> = Color::new(0.5, 0.0, 0.5);
    pub const ORANGE: Color<f32> = Color::new(1.0, 0.5, 0.0);
    pub const PINK: Color<f32> = Color::new(1.0, 0.0, 0.5);
    pub const LIME: Color<f32> = Color::new(0.0, 1.0, 0.5);
    pub const TEAL: Color<f32> = Color::new(0.5, 1.0, 0.5);
}
