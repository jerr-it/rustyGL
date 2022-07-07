use vector::Vector2;

pub trait Drawable {
    fn draw(&self);
}

pub trait Shape2D {
    /// Moves the shape in the given x, y direction
    fn translate(&mut self, translation: Vector2<f32>) -> &mut Self;

    /// Rotates the shape around its center by the given angle
    fn rotate(&mut self, angle: f32) -> &mut Self;

    /// Scales the shape by the given factor
    fn scale(&mut self, scl: f32) -> &mut Self;
}