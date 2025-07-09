use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use crate::vector::vector_3d::Vector3D;

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Matrix3D(pub [[f32; 3]; 3]);

#[warn(dead_code)]
impl Matrix3D {
    pub fn new(
        n00: f32,
        n01: f32,
        n02: f32,
        n10: f32,
        n11: f32,
        n12: f32,
        n20: f32,
        n21: f32,
        n22: f32,
    ) -> Self {
        Matrix3D([[n00, n10, n20], [n01, n11, n21], [n02, n12, n22]])
    }
    pub fn new_with_vectors(row1: Vector3D, row2: Vector3D, row3: Vector3D) -> Self {
        Matrix3D([
            [row1.x, row2.x, row3.x],
            [row1.y, row2.y, row3.y],
            [row1.z, row2.z, row3.z],
        ])
    }

    pub fn get(&self, i: usize, j: usize) -> Option<&f32> {
        if i < 3 && j < 3 {
            Some(&self.0[i][j])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, i: usize, j: usize) ->  Option<&mut f32> {
        if i < 3 && j < 3 {
            Some(&mut self.0[i][j])
        } else {
            None
        }
    }

    pub fn get_vector(&self, j: usize) -> Option<&Vector3D> {
        if j < 3 {
            unsafe { Some(&*(&self.0[j] as *const [f32; 3] as *const Vector3D)) }
        }else{
            None
        }
    }
    pub fn get_mut_vector(&mut self, j: usize) -> Option<&mut Vector3D> {
        if j < 3 {
            unsafe { Some(&mut *(&mut self.0[j] as *mut [f32; 3] as *mut Vector3D)) }
        }else{
            None
        }

    }
    pub fn transpose(&self) -> Self{
        let mut transposed = [[0.0f32;3];3];
        for i in 0..3{
            for j in 0..3{
                transposed[i][j] = self.0[j][i];
            }
        }
        Matrix3D(transposed)
    }
    pub fn transpose_mut(&mut self) -> &mut Self{
        let mut transposed = [[0.0f32;3];3];
        for i in 0..3{
            for j in 0..3{
                transposed[i][j] = self.0[j][i];
            }
        }
        self.0 = transposed;
        self
    }
}

impl Mul<Vector3D> for Matrix3D {
    type Output = Vector3D;
    fn mul(mut self, rhs: Vector3D) -> Self::Output {
        self.transpose_mut();
        self.0.iter().map(|row|{
            row.iter()
                .zip(rhs.into_iter())
                .map(|(a,b)| a * b)
                .sum::<f32>()
        }).collect::<Vec<f32>>()
        .try_into().unwrap()
    }
}

impl MulAssign<Matrix3D> for Matrix3D{
    fn mul_assign(&mut self, rhs: Matrix3D) {
        let mut rhs = rhs.transpose();
        self.0.iter_mut().for_each(|row|{
            rhs.0.iter_mut().for_each(|col|{
                row.iter_mut().zip(col.iter_mut())
                    .for_each(|(a,b)| *a *= *b);
            });
        });

    }
}
impl Mul<Matrix3D> for Matrix3D{
    type Output = Matrix3D;
    fn mul(self, rhs: Matrix3D) -> Self::Output {
        let rhs = rhs.transpose();
        self.0.iter().map(|row|{
            rhs.0.iter().map(|col|{
                row.iter()
                    .zip(col.iter())
                    .map(|(a,b)| a * b)
                    .sum::<f32>()
            }).collect::<Vec<f32>>()
            .try_into().unwrap()
        }).collect::<Vec<[f32;3]>>()
        .try_into().unwrap()

    }
}
impl AddAssign<Matrix3D> for Matrix3D{
    fn add_assign(&mut self, rhs: Matrix3D) {
        self.0.iter_mut()
            .zip(rhs.0.iter())
            .for_each(|(row_a, row_b)| {
                row_a.iter_mut()
                    .zip(row_b.iter())
                    .for_each(|(a, b)| *a += *b);
            });
    }
}

impl Add<Matrix3D> for Matrix3D{
    type Output = Matrix3D;
    fn add(self, rhs: Matrix3D) -> Self::Output {
        self.0.iter().zip(rhs.0.iter())
            .map(|(row_a, row_b)| {
                row_a.iter().zip(row_b)
                    .map(|(a,b)| *a+*b)
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap()
            }).collect::<Vec<[f32;3]>>()
            .try_into()
            .unwrap()
    }
}
impl SubAssign<Matrix3D> for Matrix3D{
    fn sub_assign(&mut self, rhs: Matrix3D) {
        self.0.iter_mut()
            .zip(rhs.0.iter())
            .for_each(|(row_a, row_b)| {
                row_a.iter_mut()
                    .zip(row_b.iter())
                    .for_each(|(a, b)| *a -= *b);
            });
    }
}
impl Sub<Matrix3D> for Matrix3D{
    type Output = Matrix3D;
    fn sub(self, rhs: Matrix3D) -> Self::Output {
        self.0.iter().zip(rhs.0.iter())
            .map(|(row_a, row_b)| {
                row_a.iter().zip(row_b)
                    .map(|(a,b)| *a-*b)
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap()
            }).collect::<Vec<[f32;3]>>()
            .try_into()
            .unwrap()
    }
}
impl MulAssign<f32> for Matrix3D{
    fn mul_assign(&mut self, rhs: f32) {
        self.0.iter_mut().for_each(|row|{
            row.iter_mut().for_each(|a| *a*=rhs)
        })
    }
}
impl Mul<f32> for Matrix3D{
    type Output = Matrix3D;
    fn mul(self, rhs: f32) -> Self::Output {
        self.0.iter().map(|row|{
            row.iter().map(|a| *a*rhs)
                .collect::<Vec<f32>>()
                .try_into()
                .unwrap()
        }).collect::<Vec<[f32;3]>>()
        .try_into().unwrap()
    }
}

impl DivAssign<f32> for Matrix3D{
    fn div_assign(&mut self, rhs: f32) {
        let inv = 1.0 / rhs;
        *self *= inv
    }
}

impl Div<f32> for Matrix3D{
    type Output = Matrix3D;
    fn div(self, rhs: f32) -> Self::Output {
        let inv = 1.0 / rhs;
        self * inv

    }
}

impl TryFrom<Vec<[f32;3]>> for Matrix3D{
    type Error = & 'static str;
    fn try_from(value: Vec<[f32;3]>) -> Result<Self, Self::Error> {
        if value.len() != 3 {
            return Err("Expected Vec with exactly 3 rows");
        }

        let array: [[f32;3];3] = value
            .try_into()
            .map_err(|_| "Failed to convert Vec to 3x3 array")?;
        Ok(Matrix3D(array))
    }
}

#[cfg(test)]
mod test_matrix_3d {
    use super::*;

    #[test]
    fn new() {
        let m = Matrix3D::new(1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0);
        assert_eq!(
            m.0,
            [[1.0, 1.0, 1.0], [1.0, 1.0, 1.0], [1.0, 1.0, 1.0]]
        );
    }
    #[test]
    fn new_with_vectors() {
        let v = Vector3D::new(1.0, 1.0, 1.0);
        let v2 = Vector3D::new(1.0, 1.0, 1.0);
        let v3 = Vector3D::new(1.0, 1.0, 1.0);
        let m = Matrix3D::new_with_vectors(v, v2, v3);
        assert_eq!(
            m.0,
            [[1.0, 1.0, 1.0], [1.0, 1.0, 1.0], [1.0, 1.0, 1.0]]
        );
    }
    #[test]
    fn get() {
        let m = Matrix3D::new(1.0, 3.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0);
        assert_eq!(m.get(1, 0), Some(&3.0));
    }
    #[test]
    fn get_index_out_of_bound(){
        let m = Matrix3D::new(1.0, 3.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0);
        assert!(m.get(3, 1).is_none());
        assert!(m.get(1, 3).is_none());
    }
    #[test]
    fn get_mutable_index_out_of_bound(){
        let mut m = Matrix3D::new(1.0, 3.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0);
        assert!(m.get_mut(3, 1).is_none());
        assert!(m.get_mut(1, 3).is_none());
    }
    #[test]
    fn get_mutable() {
        let mut m = Matrix3D::new(1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0);
        if let Some(val) = m.get_mut(1, 0){
            *val = 3.0;
        }
        assert_eq!(m.get_mut(1, 0), Some(&mut 3.0));

    }
    #[test]
    fn get_vector() {
        let m = Matrix3D::new(
            1.0, 2.0, 3.0,
            4.0, 5.0, 6.0,
            7.0, 8.0, 9.0);
        let v = m.get_vector(0);
        let expected = Vector3D::new(1.0,4.0,7.0);
        assert_eq!(v, Some(&expected));
    }
    #[test]
    fn get_mutable_vector() {
        let mut m = Matrix3D::new(
            1.0, 2.0, 3.0,
            4.0, 5.0, 6.0,
            7.0, 8.0, 9.0);
        if let Some(val) = m.get_mut_vector(0){
            val.y = 20.0;
        }
        let mut expected = Vector3D::new(1.0,20.0,7.0);
        assert_eq!(m.get_mut_vector(0), Some(&mut expected));

    }
    #[test]
    fn get_vector_index_out_of_bound(){
        let mut m = Matrix3D::new(
            1.0, 2.0, 3.0,
            4.0, 5.0, 6.0,
            7.0, 8.0, 9.0);

        assert!(m.get_vector(4).is_none());
        assert!(m.get_mut_vector(10).is_none());
    }
    #[test]
    fn add_assign(){
        let mut m = Matrix3D::new(1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0);
        let m2 = Matrix3D::new(1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0);
        let res = Matrix3D::new(2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0);
        m += m2;
        assert_eq!(m, res);
    }
    #[test]
    fn add(){
        let  m = Matrix3D::new(1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0);
        let m2 = Matrix3D::new(1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0);
        let res = Matrix3D::new(2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0);
        let m = m + m2;
        assert_eq!(m, res);
    }
    #[test]
    fn sub_assign(){
        let mut m = Matrix3D::new(3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0);
        let m2 = Matrix3D::new(1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0);
        let res = Matrix3D::new(2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0);
        m -= m2;
        assert_eq!(m, res);
    }
    #[test]
    fn sub(){
        let m = Matrix3D::new(3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0);
        let m2 = Matrix3D::new(1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0);
        let res = Matrix3D::new(2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0);
        let m = m - m2;
        assert_eq!(m, res);
    }
    #[test]
    fn mul_assign_f32(){
        let mut m = Matrix3D::new(3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0);
        let res = Matrix3D::new(6.0, 6.0, 6.0, 6.0, 6.0, 6.0, 6.0, 6.0, 6.0);
        m *=2.0;
        assert_eq!(m, res);
    }
    #[test]
    fn mul_f32(){
        let  m = Matrix3D::new(3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0);
        let res = Matrix3D::new(6.0, 6.0, 6.0, 6.0, 6.0, 6.0, 6.0, 6.0, 6.0);
        let m = m *2.0;
        assert_eq!(m, res);
    }
    #[test]
    fn div_assign_f32(){
        let res = Matrix3D::new(3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0);
        let mut m = Matrix3D::new(6.0, 6.0, 6.0, 6.0, 6.0, 6.0, 6.0, 6.0, 6.0);
        m /=2.0;
        assert_eq!(m, res);
    }
    #[test]
    fn div_f32(){
        let res = Matrix3D::new(3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0);
        let m = Matrix3D::new(6.0, 6.0, 6.0, 6.0, 6.0, 6.0, 6.0, 6.0, 6.0);
        let m = m /2.0;
        assert_eq!(m, res);
    }
    #[test]
    fn try_into_vec_arr_f32_size_3_into_matrix3d(){
        let res = Matrix3D::new(3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0);
        let m_vec:Vec<[f32; 3]> = vec![[3.0,3.0,3.0],[3.0,3.0,3.0],[3.0,3.0,3.0]];
        let m:Matrix3D = m_vec.try_into().expect("Failed to transfor the Vec<[f32;3]> into a Matrix3D");
        assert_eq!(m,res);

    }
    #[test]
    fn mul_assign_matrix(){
        todo!()
    }
    #[test]
    fn mul_matrix(){
        todo!()
    }
    #[test]
    fn mul_matrix_with_vector(){
        let m = Matrix3D::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        let v = Vector3D::new(1.0, 2.0, 3.0);
        let res = Vector3D::new(14.0,32.0,50.0);
        let mul = m * v;
        assert_eq!(mul,res);
    }
}
