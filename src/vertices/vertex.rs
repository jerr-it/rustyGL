use crate::{vector::{Vector2, Vector3}, Color};

/// |-f32-|-f32-|-f32-|-f32-|-f32-|-f32-|-f32-|-f32-|  <br/>
/// |-----Position----|------Color------|-UV-coords-|  
#[repr(C)]
#[derive(Debug)]
pub struct Vertex {
    position: Vector3<f32>,
    color: Color<f32>,
    texture_coord: Vector2<f32>,
}

impl Vertex {
    pub fn new(position: Vector3<f32>, color: Color<f32>, texture_coord: Vector2<f32>) -> Vertex {
        Vertex { position, color, texture_coord }
    }

    pub fn position(&self) -> &Vector3<f32> {
        &self.position
    }

    pub fn position_mut(&mut self) -> &mut Vector3<f32> {
        &mut self.position
    }

    pub fn color(&self) -> &Color<f32> {
        &self.color
    }

    pub fn color_mut(&mut self) -> &mut Color<f32> {
        &mut self.color
    }

    pub fn texture_coord(&self) -> &Vector2<f32> {
        &self.texture_coord
    }

    pub fn texture_coord_mut(&mut self) -> &mut Vector2<f32> {
        &mut self.texture_coord
    }
}