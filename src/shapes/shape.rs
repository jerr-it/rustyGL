use crate::vector::Vector2;

pub trait Drawable {
    fn draw(&self);
}

pub trait Shape2D {
    fn translate(&mut self, translation: Vector2<f32>);
    fn rotate(&mut self, angle: f32);
    fn scale(&mut self, scl: f32);
}