use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign};

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Vector4<T> {
    x: T,
    y: T,
    z: T,
    w: T,
}

impl<T: Copy> Vector4<T> {
    pub fn new(x: T, y: T, z: T, w: T) -> Vector4<T>{
        Vector4 { x, y, z, w }
    }

    pub fn components(&self) -> (&T, &T, &T, &T) {
        (&self.x, &self.y, &self.z, &self.w)
    }

    pub fn components_mut(&mut self) -> (&mut T, &mut T, &mut T, &mut T) {
        (&mut self.x, &mut self.y, &mut self.z, &mut self.w)
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