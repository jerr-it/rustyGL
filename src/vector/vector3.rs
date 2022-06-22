use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign};

use super::Vector4;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Default + Copy> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Vector3<T>{
        Vector3 { x, y, z }
    }

    pub fn as_vector4(&self) -> Vector4<T> {
        Vector4::new(self.x, self.y, self.z, Default::default())
    }
}

impl<T: Add<Output = T>> Add<Vector3<T>> for Vector3<T> {
    type Output = Vector3<T>;

    fn add(self, rhs: Vector3<T>) -> Self::Output {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl<T: AddAssign> AddAssign<Vector3<T>> for Vector3<T> {
    fn add_assign(&mut self, rhs: Vector3<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<T: Sub<Output = T>> Sub<Vector3<T>> for Vector3<T> {
    type Output = Vector3<T>;

    fn sub(self, rhs: Vector3<T>) -> Self::Output {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

impl<T: SubAssign> SubAssign<Vector3<T>> for Vector3<T> {
    fn sub_assign(&mut self, rhs: Vector3<T>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for Vector3<T> {
    type Output = Vector3<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Vector3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs
        }
    }
}

impl<T: MulAssign + Copy> MulAssign<T> for Vector3<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector3_new() {
        let v = Vector3::new(1.0, 2.0, 3.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
    }

    #[test]
    fn test_vector3_as_vector4() {
        let v = Vector3::new(1.0, 2.0, 3.0);
        let v4 = v.as_vector4();
        assert_eq!(v4.x, 1.0);
        assert_eq!(v4.y, 2.0);
        assert_eq!(v4.z, 3.0);
        assert_eq!(v4.w, 0.0);
    }

    #[test]
    fn test_vector3_add() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(3.0, 4.0, 5.0);
        let v3 = v1 + v2;
        assert_eq!(v3.x, 4.0);
        assert_eq!(v3.y, 6.0);
        assert_eq!(v3.z, 8.0);
    }

    #[test]
    fn test_vector3_add_assign() {
        let mut v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(3.0, 4.0, 3.0);
        v1 += v2;
        assert_eq!(v1.x, 4.0);
        assert_eq!(v1.y, 6.0);
        assert_eq!(v1.z, 6.0);
    }

    #[test]
    fn test_vector3_sub() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(3.0, 4.0, 3.0);
        let v3 = v1 - v2;
        assert_eq!(v3.x, -2.0);
        assert_eq!(v3.y, -2.0);
        assert_eq!(v3.z, 0.0);
    }

    #[test]
    fn test_vector3_sub_assign() {
        let mut v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(3.0, 4.0, 1.0);
        v1 -= v2;
        assert_eq!(v1.x, -2.0);
        assert_eq!(v1.y, -2.0);
        assert_eq!(v1.z, 2.0);
    }

    #[test]
    fn test_vector3_mul() {
        let v1 = Vector3::new(1.0, 2.0, 4.0);
        let v2 = v1 * 2.0;
        assert_eq!(v2.x, 2.0);
        assert_eq!(v2.y, 4.0);
        assert_eq!(v2.z, 8.0);
    }

    #[test]
    fn test_vector3_mul_assign() {
        let mut v1 = Vector3::new(1.0, 2.0, 3.0);
        v1 *= 2.0;
        assert_eq!(v1.x, 2.0);
        assert_eq!(v1.y, 4.0);
        assert_eq!(v1.z, 6.0);
    }
}