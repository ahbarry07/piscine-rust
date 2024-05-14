pub use lalgebra_scalar::*;
use std::ops::{Add, Sub};

#[derive(Debug, PartialEq, Eq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Scalar + Add<Output = T> + Copy> Add for Matrix<T> {
    type Output = Option<Matrix<T>>;

    fn add(self, other: Matrix<T>) -> Option<Matrix<T>> {
        if self.0.len() != other.0.len() || self.0[0].len() != other.0[0].len() {
            return None; 
        }

        let mut result = Vec::with_capacity(self.0.len());
        for i in 0..self.0.len() {
            let mut row = Vec::with_capacity(self.0[i].len());
            for j in 0..self.0[i].len() {
                row.push(self.0[i][j] + other.0[i][j]);
            }
            result.push(row);
        }
        Some(Matrix(result))
    }
}

impl<T: Scalar + Sub<Output = T> + Copy> Sub for Matrix<T> {
    type Output = Option<Matrix<T>>;

    fn sub(self, other: Matrix<T>) -> Option<Matrix<T>> {
        if self.0.len() != other.0.len() || self.0[0].len() != other.0[0].len() {
            return None;
        }
       
        let mut result = Vec::with_capacity(self.0.len());
        for i in 0..self.0.len() {
            let mut row = Vec::with_capacity(self.0[i].len());
            for j in 0..self.0[i].len() {
                row.push(self.0[i][j] - other.0[i][j]);
            }
            result.push(row);
        }
        Some(Matrix(result))
    }
}