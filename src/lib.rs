pub mod debug;
mod gpu;
mod shader;
pub mod shapes;
mod ssbo;
pub mod vertices;
mod windows;

pub use gpu::GPU;
pub use shader::{ComputeShader, PipelineShader, ShaderSource};
pub use ssbo::SSBO;
pub use windows::Window;

pub type Color<T> = vector::Vector3<T>;
