use std::ops::{Add, Mul};

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T> Matrix<T>
where
    T: Clone,
{
    pub fn number_of_cols(&self) -> usize {
        self.0[0].len()
    }

    pub fn number_of_rows(&self) -> usize {
        self.0.len()
    }

    pub fn row(&self, n: usize) -> Vec<T> {
        self.0[n].clone()
    }

    pub fn col(&self, n: usize) -> Vec<T> {
        self.0.iter().map(|row| row[n].clone()).collect()
    }
}

impl<T> Mul for Matrix<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T>,
{
    type Output = Option<Matrix<T>>;
    fn mul(self, other: Matrix<T>) -> Option<Matrix<T>> {
        if self.0.len() != other.0.len() || self.0[0].len() != other.0[0].len() {
            return None;
        }
        let mut res = Vec::new();
        for i in 0..self.0.len() {
            let mut r = Vec::new();
            for j in 0..other.0[0].len() {
                let mut sum = self.0[i][0].clone() * other.0[0][j].clone();
                for k in 1..self.0[0].len() {
                    sum = sum + self.0[i][k].clone() * other.0[k][j].clone();
                }
                r.push(sum);
            }
            res.push(r);
        }
        Some(Matrix(res))
    }
}
