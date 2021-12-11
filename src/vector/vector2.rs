pub use std::ops;

use super::vector3::Vector3;

#[repr(C)]
pub struct Vector2 {
    x: f32,
    y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Vector2 {
        Vector2 { x, y }
    }

    pub fn as_vector3(&self) -> Vector3 {
        Vector3::new(self.x, self.y, 0.0)
    }
}

impl ops::Add<Vector2> for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: Vector2) -> Self::Output {
        Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Sub<Vector2> for Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: Vector2) -> Self::Output {
        Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::Mul<f32> for Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl ops::AddAssign<Vector2> for Vector2 {
    fn add_assign(&mut self, rhs: Vector2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl ops::SubAssign<Vector2> for Vector2 {
    fn sub_assign(&mut self, rhs: Vector2) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl ops::MulAssign<f32> for Vector2 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}