use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign};

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Vector4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T: Copy> Vector4<T> {
    pub fn new(x: T, y: T, z: T, w: T) -> Vector4<T>{
        Vector4 { x, y, z, w }
    }
}

impl<T: Add<Output = T>> Add<Vector4<T>> for Vector4<T> {
    type Output = Vector4<T>;

    fn add(self, rhs: Vector4<T>) -> Self::Output {
        Vector4 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w
        }
    }
}

impl<T: AddAssign> AddAssign<Vector4<T>> for Vector4<T> {
    fn add_assign(&mut self, rhs: Vector4<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self.w += rhs.w;
    }
}

impl<T: Sub<Output = T>> Sub<Vector4<T>> for Vector4<T> {
    type Output = Vector4<T>;

    fn sub(self, rhs: Vector4<T>) -> Self::Output {
        Vector4 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w
        }
    }
}

impl<T: SubAssign> SubAssign<Vector4<T>> for Vector4<T> {
    fn sub_assign(&mut self, rhs: Vector4<T>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self.w -= rhs.w;
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for Vector4<T> {
    type Output = Vector4<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Vector4 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs
        }
    }
}

impl<T: MulAssign + Copy> MulAssign<T> for Vector4<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
        self.w *= rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector4_new() {
        let vector4 = Vector4::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(vector4.x, 1.0);
        assert_eq!(vector4.y, 2.0);
        assert_eq!(vector4.z, 3.0);
        assert_eq!(vector4.w, 4.0);
    }

    #[test]
    fn test_vector4_add() {
        let vector4 = Vector4::new(1.0, 2.0, 3.0, 4.0);
        let vector4_add = Vector4::new(2.0, 4.0, 6.0, 8.0);
        let vector4_result = vector4 + vector4_add;
        assert_eq!(vector4_result.x, 3.0);
        assert_eq!(vector4_result.y, 6.0);
        assert_eq!(vector4_result.z, 9.0);
        assert_eq!(vector4_result.w, 12.0);
    }

    #[test]
    fn test_vector4_add_assign() {
        let mut vector4 = Vector4::new(1.0, 2.0, 3.0, 4.0);
        let vector4_add = Vector4::new(2.0, 4.0, 6.0, 8.0);
        vector4 += vector4_add;
        assert_eq!(vector4.x, 3.0);
        assert_eq!(vector4.y, 6.0);
        assert_eq!(vector4.z, 9.0);
        assert_eq!(vector4.w, 12.0);
    }

    #[test]
    fn test_vector4_sub() {
        let vector4 = Vector4::new(1.0, 2.0, 3.0, 4.0);
        let vector4_sub = Vector4::new(2.0, 4.0, 6.0, 8.0);
        let vector4_result = vector4 - vector4_sub;
        assert_eq!(vector4_result.x, -1.0);
        assert_eq!(vector4_result.y, -2.0);
        assert_eq!(vector4_result.z, -3.0);
        assert_eq!(vector4_result.w, -4.0);
    }

    #[test]
    fn test_vector4_sub_assign() {
        let mut vector4 = Vector4::new(1.0, 2.0, 3.0, 4.0);
        let vector4_sub = Vector4::new(2.0, 4.0, 6.0, 8.0);
        vector4 -= vector4_sub;
        assert_eq!(vector4.x, -1.0);
        assert_eq!(vector4.y, -2.0);
        assert_eq!(vector4.z, -3.0);
        assert_eq!(vector4.w, -4.0);
    }

    #[test]
    fn test_vector4_mul() {
        let vector4 = Vector4::new(1.0, 2.0, 3.0, 4.0);
        let vector4_mul = 2.0;
        let vector4_result = vector4 * vector4_mul;
        assert_eq!(vector4_result.x, 2.0);
        assert_eq!(vector4_result.y, 4.0);
        assert_eq!(vector4_result.z, 6.0);
        assert_eq!(vector4_result.w, 8.0);
    }

    #[test]
    fn test_vector4_mul_assign() {
        let mut vector4 = Vector4::new(1.0, 2.0, 3.0, 4.0);
        let vector4_mul = 2.0;
        vector4 *= vector4_mul;
        assert_eq!(vector4.x, 2.0);
        assert_eq!(vector4.y, 4.0);
        assert_eq!(vector4.z, 6.0);
        assert_eq!(vector4.w, 8.0);
    }
}