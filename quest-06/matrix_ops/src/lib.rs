// use crate::{Matrix, Scalar};
use lalgebra_scalar::Scalar;
// use matrix::Matrix;
use std::ops::{Add, AddAssign, Sub, SubAssign};

// COPY PASTE THE CODE BELOW TO MAKE IT WORK
#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Scalar<Item = T> + Clone> Matrix<T> {
    pub fn new() -> Matrix<T> {
        Matrix(vec![vec![T::zero(); 1]])
    }

    pub fn zero(row: usize, col: usize) -> Matrix<T> {
        Matrix(vec![vec![T::zero(); col]; row])
    }

    pub fn identity(n: usize) -> Matrix<T> {
        let mut matrix = Matrix::zero(n, n);
        for i in 0..n {
            matrix.0[i][i] = T::one();
        }
        matrix
    }
}

impl<T> Add for Matrix<T>
where
    T: Scalar + Scalar<Item = T> + Clone + Add<Output = T> + AddAssign,
{
    type Output = Option<Matrix<T>>;
    fn add(self, other: Matrix<T>) -> Self::Output {
        if self.0.len() != other.0.len() || self.0[0].len() != other.0[0].len() {
            return None;
        }

        let mut res = Matrix::<T>::zero(self.0.len(), self.0[0].len());
        for row in 0..self.0.len() {
            for col in 0..self.0[0].len() {
                res.0[row][col] = self.0[row][col].clone() + other.0[row][col].clone();
            }
        }
        Some(res)
    }
}

impl<T> Sub for Matrix<T>
where
    T: Scalar + Scalar<Item = T> + Clone + Sub<Output = T> + SubAssign,
{
    type Output = Option<Matrix<T>>;
    fn sub(self, other: Matrix<T>) -> Option<Matrix<T>> {
        if self.0.len() != other.0.len() || self.0[0].len() != other.0[0].len() {
            return None;
        }

        let mut res = Matrix::<T>::zero(self.0.len(), self.0[0].len());
        for row in 0..self.0.len() {
            for col in 0..self.0[0].len() {
                res.0[row][col] = self.0[row][col].clone() - other.0[row][col].clone();
            }
        }
        Some(res)
    }
}

// --------------------------------------------------------------------------------------------------------------

// THIS SHOULD WORK BUT IT DOESN'T DUE TO A BUG IN THE MAIN FUNCTION

// trait MatrixAdd<T> {
//     fn add(&self, other: &Matrix<T>) -> Option<Matrix<T>>;
// }

// impl<T> MatrixAdd<T> for Matrix<T>
// where
//     T: Scalar + Scalar<Item = T> + Clone + Add<Output = T> + AddAssign,
// {
//     fn add(&self, other: &Matrix<T>) -> Option<Matrix<T>> {
//         if self.0.len() != other.0.len() || self.0[0].len() != other.0[0].len() {
//             return None;
//         }

//         let mut res = Matrix::<T>::zero(self.0.len(), self.0[0].len());
//         for row in 0..self.0.len() {
//             for col in 0..self.0[0].len() {
//                 res.0[row][col] = self.0[row][col].clone() + other.0[row][col].clone();
//             }
//         }
//         Some(res)
//     }
// }

//--------------------------------------------------------------------------------------------------------------------
