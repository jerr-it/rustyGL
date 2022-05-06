pub mod debug;
mod ebo;
mod gpu;
mod shader;
mod ssbo;
mod vao;
mod vbo;
pub mod vector;

pub use ebo::EBO;
pub use gpu::GPU;
pub use shader::{ComputeShader, PipelineShader, ShaderSource};
pub use ssbo::SSBO;
pub use vao::VAO;
pub use vbo::VBO;

pub type Color<T> = vector::Vector3<T>;
