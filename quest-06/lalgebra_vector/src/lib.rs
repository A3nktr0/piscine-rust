use lalgebra_scalar::Scalar;
use std::iter::Sum;
use std::ops::{Add, Mul};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Vector<T: Scalar>(pub Vec<T>);

// Implement the Add trait for the Vector struct
// allowing two vectors to be added together
impl<T> Add for Vector<T>
where
    T: Scalar + Clone + Add<Output = T>,
{
    type Output = Option<Self>;

    fn add(self, other: Self) -> Option<Self> {
        if self.0.len() != other.0.len() {
            return None;
        }

        let mut result = Vec::new();
        for i in 0..self.0.len() {
            result.push(self.0[i].clone() + other.0[i].clone());
        }
        Some(Vector(result))
    }
}

// Implement utilities for the Vector
impl<T> Vector<T>
where
    T: Scalar + Mul<Output = T> + Add<Output = T> + Clone + Sum<T>,
{
    pub fn new() -> Self {
        Vector(Vec::new())
    }

    pub fn dot(&self, other: &Self) -> Option<T> {
        if self.0.len() != other.0.len() {
            return None;
        }

        Some(
            self.0
                .iter()
                .zip(other.0.iter())
                .map(|(a, b)| a.clone() * b.clone())
                .sum(),
        )
    }
}
