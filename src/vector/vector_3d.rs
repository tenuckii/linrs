use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Vector3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[warn(dead_code)]
impl Vector3D {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector3D { x, y, z }
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut f32> {
        match index {
            0 => Some(&mut self.x),
            1 => Some(&mut self.y),
            2 => Some(&mut self.z),
            _ => None,
        }
    }
    pub fn get(&self, index: usize) -> Option<&f32> {
        match index {
            0 => Some(&self.x),
            1 => Some(&self.y),
            2 => Some(&self.z),
            _ => None,
        }
    }
    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    pub fn normalize(&self) -> Self {
        *self / self.magnitude()
    }
}
//Vector3D operations
impl AddAssign<Vector3D> for Vector3D {
    fn add_assign(&mut self, rhs: Vector3D) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}
impl Add<Vector3D> for Vector3D {
    type Output = Vector3D;
    fn add(self, rhs: Vector3D) -> Self::Output {
        Vector3D::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}
impl SubAssign<Vector3D> for Vector3D {
    fn sub_assign(&mut self, rhs: Vector3D) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}
impl Sub<Vector3D> for Vector3D {
    type Output = Vector3D;
    fn sub(self, rhs: Vector3D) -> Self::Output {
        Vector3D::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

//f32 operations
impl MulAssign<f32> for Vector3D {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}
impl Mul<f32> for Vector3D {
    type Output = Vector3D;
    fn mul(self, rhs: f32) -> Self::Output {
        Vector3D::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}
impl DivAssign<f32> for Vector3D {
    fn div_assign(&mut self, rhs: f32) {
        let inv = 1.0 / rhs;
        *self *= inv;
    }
}

impl Div<f32> for Vector3D {
    type Output = Vector3D;
    fn div(self, rhs: f32) -> Self::Output {
        let inv = 1.0 / rhs;
        self * inv
    }
}
impl AddAssign<f32> for Vector3D {
    fn add_assign(&mut self, rhs: f32) {
        self.x += rhs;
        self.y += rhs;
        self.z += rhs;
    }
}
impl Add<f32> for Vector3D {
    type Output = Vector3D;
    fn add(self, rhs: f32) -> Self::Output {
        Vector3D::new(self.x + rhs, self.y + rhs, self.z + rhs)
    }
}
impl SubAssign<f32> for Vector3D {
    fn sub_assign(&mut self, rhs: f32) {
        self.x -= rhs;
        self.y -= rhs;
        self.z -= rhs;
    }
}
impl Sub<f32> for Vector3D {
    type Output = Vector3D;
    fn sub(self, rhs: f32) -> Self::Output {
        Vector3D::new(self.x - rhs, self.y - rhs, self.z - rhs)
    }
}
impl Neg for Vector3D {
    type Output = Vector3D;
    fn neg(self) -> Self::Output {
        Vector3D::new(-self.x, -self.y, -self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_vector() {
        let v = Vector3D::new(1.0, 2.0, 3.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
    }

    #[test]
    fn get_index() {
        let v = Vector3D::new(1.0, 2.0, 3.0);
        assert_eq!(v.get(0), Some(&1.0));
        assert_eq!(v.get(1), Some(&2.0));
        assert_eq!(v.get(2), Some(&3.0));
    }

    #[test]
    fn get_mutable_index() {
        let mut v = Vector3D::new(1.0, 2.0, 3.0);
        if let Some(val) = v.get_mut(1) {
            *val = 20.0;
        }
        assert_eq!(v.get(1), Some(&20.0));
    }

    #[test]
    fn out_of_bound() {
        let mut v = Vector3D::default();
        assert!(v.get(10).is_none());
        assert!(v.get_mut(10).is_none());
    }

    #[test]
    fn mul_assign_f32() {
        let mut v = Vector3D::new(1.0, 1.0, 1.0);
        v *= 2.0;
        assert_eq!(v.x, 2.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 2.0);
    }

    #[test]
    fn mul_f32() {
        let v = Vector3D::new(1.0, 1.0, 1.0);
        let v2 = v * 2.0;
        assert_eq!(v2.x, 2.0);
        assert_eq!(v2.y, 2.0);
        assert_eq!(v2.z, 2.0);
    }

    #[test]
    fn div_assign_f32() {
        let mut v = Vector3D::new(1.0, 1.0, 1.0);
        v /= 2.0;
        assert_eq!(v.x, 0.5);
        assert_eq!(v.y, 0.5);
        assert_eq!(v.z, 0.5);
    }
    #[test]
    fn div_f32() {
        let v = Vector3D::new(1.0, 1.0, 1.0);
        let v2 = v / 2.0;
        assert_eq!(v2.x, 0.5);
        assert_eq!(v2.y, 0.5);
        assert_eq!(v2.z, 0.5);
    }
    #[test]
    fn add_assign_f32() {
        let mut v = Vector3D::new(1.0, 1.0, 1.0);
        v += 2.0;
        assert_eq!(v.x, 3.0);
        assert_eq!(v.y, 3.0);
        assert_eq!(v.z, 3.0);
    }
    #[test]
    fn add_f32() {
        let v = Vector3D::new(1.0, 1.0, 1.0);
        let v2 = v + 2.0;
        assert_eq!(v2.x, 3.0);
        assert_eq!(v2.y, 3.0);
        assert_eq!(v2.z, 3.0);
    }
    #[test]
    fn sub_assign_f32() {
        let mut v = Vector3D::new(1.0, 1.0, 1.0);
        v -= 2.0;
        assert_eq!(v.x, -1.0);
        assert_eq!(v.y, -1.0);
        assert_eq!(v.z, -1.0);
    }
    #[test]
    fn sub_f32() {
        let v = Vector3D::new(1.0, 1.0, 1.0);
        let v2 = v - 2.0;
        assert_eq!(v2.x, -1.0);
        assert_eq!(v2.y, -1.0);
        assert_eq!(v2.z, -1.0);
    }
    #[test]
    fn neg() {
        let v = Vector3D::new(1.0, 1.0, 1.0);
        let v2 = -v;
        assert_eq!(v2.x, -1.0);
        assert_eq!(v2.y, -1.0);
        assert_eq!(v2.z, -1.0);
    }
    #[test]
    fn magnitude() {
        let v = Vector3D::new(1.0, 2.0, 3.0);
        let mag: f32 = (1.0 * 1.0 + 2.0 * 2.0 + 3.0 * 3.0) as f32;
        assert_eq!(v.magnitude(), mag.sqrt())
    }
    #[test]
    fn normalize() {
        let v = Vector3D::new(1.0, 2.0, 3.0);
        let mag: f32 = (1.0 * 1.0 + 2.0 * 2.0 + 3.0 * 3.0) as f32;
        let nor = v / mag.sqrt();
        assert_eq!(v.normalize(), nor)
    }

    #[test]
    fn add_assign_vector3d() {
        let mut v = Vector3D::new(2.0, 2.0, 2.0);
        v += Vector3D::new(2.0, 2.0, 2.0);
        let vresp = Vector3D::new(4.0, 4.0, 4.0);
        assert_eq!(v, vresp)
    }
    #[test]
    fn add_vector3d() {
        let v = Vector3D::new(2.0, 2.0, 2.0);
        let v2 = v + Vector3D::new(2.0, 2.0, 2.0);
        let vresp = Vector3D::new(4.0, 4.0, 4.0);
        assert_eq!(v2, vresp)
    }
    #[test]
    fn sub_assign_vector3d() {
        let mut v = Vector3D::new(2.0, 2.0, 2.0);
        v -= Vector3D::new(1.0, 1.0, 1.0);
        let vresp = Vector3D::new(1.0, 1.0, 1.0);
        assert_eq!(v, vresp)
    }
    #[test]
    fn sub_vector3d() {
        let v = Vector3D::new(2.0, 2.0, 2.0);
        let v2 = v - Vector3D::new(1.0, 1.0, 1.0);
        let vresp = Vector3D::new(1.0, 1.0, 1.0);
        assert_eq!(v2, vresp)
    }
}
