use crate::vector::Vector2;

pub trait Drawable {
    fn draw(&self);
}

pub trait Shape2D {
    fn translate(&mut self, translation: Vector2<f32>) -> &mut Self;
    fn rotate(&mut self, angle: f32) -> &mut Self;
    fn scale(&mut self, scl: f32) -> &mut Self;
}