pub mod debug;
mod gpu;
mod shader;
mod ssbo;
pub mod shapes;
pub mod vertices;

pub use gpu::GPU;
pub use shader::{ComputeShader, PipelineShader, ShaderSource};
pub use ssbo::SSBO;

pub type Color<T> = vector::Vector3<T>;
