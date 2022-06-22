use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

use super::{Vector3, Vector4};

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

impl<T: Default + Copy> Vector2<T> {
    pub fn new(x: T, y: T) -> Vector2<T> {
        Vector2 { x, y }
    }

    pub fn as_vector3(&self) -> Vector3<T> {
        Vector3::new(self.x, self.y, Default::default())
    }

    pub fn as_vector4(&self) -> Vector4<T> {
        Vector4::new(self.x, self.y, Default::default(), Default::default())
    }
}

impl<T: Add<Output = T>> Add<Vector2<T>> for Vector2<T> {
    type Output = Vector2<T>;

    fn add(self, rhs: Vector2<T>) -> Self::Output {
        Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: AddAssign> AddAssign<Vector2<T>> for Vector2<T> {
    fn add_assign(&mut self, rhs: Vector2<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: Sub<Output = T>> Sub<Vector2<T>> for Vector2<T> {
    type Output = Vector2<T>;

    fn sub(self, rhs: Vector2<T>) -> Self::Output {
        Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: SubAssign> SubAssign<Vector2<T>> for Vector2<T> {
    fn sub_assign(&mut self, rhs: Vector2<T>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for Vector2<T> {
    type Output = Vector2<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Vector2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T: MulAssign + Copy> MulAssign<T> for Vector2<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector2_new() {
        let vector = Vector2::new(1.0, 2.0);
        assert_eq!(vector.x, 1.0);
        assert_eq!(vector.y, 2.0);
    }

    #[test]
    fn test_vector2_as_vector3() {
        let vector = Vector2::new(1.0, 2.0);
        let vector3 = vector.as_vector3();
        assert_eq!(vector3.x, 1.0);
        assert_eq!(vector3.y, 2.0);
        assert_eq!(vector3.z, 0.0);
    }

    #[test]
    fn test_vector2_as_vector4() {
        let vector = Vector2::new(1.0, 2.0);
        let vector4 = vector.as_vector4();
        assert_eq!(vector4.x, 1.0);
        assert_eq!(vector4.y, 2.0);
        assert_eq!(vector4.z, 0.0);
        assert_eq!(vector4.w, 0.0);
    }

    #[test]
    fn test_vector2_add() {
        let vector = Vector2::new(1.0, 2.0);
        let vector2 = Vector2::new(3.0, 4.0);
        let vector3 = vector + vector2;
        assert_eq!(vector3.x, 4.0);
        assert_eq!(vector3.y, 6.0);
    }

    #[test]
    fn test_vector2_add_assign() {
        let mut vector = Vector2::new(1.0, 2.0);
        let vector2 = Vector2::new(3.0, 4.0);
        vector += vector2;
        assert_eq!(vector.x, 4.0);
        assert_eq!(vector.y, 6.0);
    }

    #[test]
    fn test_vector2_sub() {
        let vector = Vector2::new(1.0, 2.0);
        let vector2 = Vector2::new(3.0, 4.0);
        let vector3 = vector - vector2;
        assert_eq!(vector3.x, -2.0);
        assert_eq!(vector3.y, -2.0);
    }

    #[test]
    fn test_vector2_sub_assign() {
        let mut vector = Vector2::new(1.0, 2.0);
        let vector2 = Vector2::new(3.0, 4.0);
        vector -= vector2;
        assert_eq!(vector.x, -2.0);
        assert_eq!(vector.y, -2.0);
    }

    #[test]
    fn test_vector2_mul() {
        let vector = Vector2::new(1.0, 2.0);
        let vector2 = vector * 2.0;
        assert_eq!(vector2.x, 2.0);
        assert_eq!(vector2.y, 4.0);
    }

    #[test]
    fn test_vector2_mul_assign() {
        let mut vector = Vector2::new(1.0, 2.0);
        vector *= 2.0;
        assert_eq!(vector.x, 2.0);
        assert_eq!(vector.y, 4.0);
    }
}
