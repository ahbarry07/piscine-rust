use lalgebra_scalar::*;
use std::ops::Mul;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl <T: Copy> Matrix<T> {
	pub fn number_of_cols(&self) -> usize {
		self.0.len()
	}

	pub fn number_of_rows(&self) -> usize {
		self.0[0].len()
	}

	pub fn row(&self, n: usize) -> Vec<T> {
		self.0[n].clone()
	}

	pub fn col(&self, n: usize) -> Vec<T> {
		let mut vec: Vec<T> = Vec::new();
		for ind in 0..self.0.len(){
			vec.push(self.0[ind][n])
		}
		vec
	}
}

impl <T: Scalar<Item = T> + Mul<Output = T> + Copy + std::ops::AddAssign> Mul for Matrix<T> {
	type Output = Option<Matrix<T>>;

	fn mul(self, rhs: Self) -> Self::Output {
		println!("self: {:?}, rhs: {:?}", self, rsh);
		let mut result: Vec<Vec<T>> = vec![vec![T::zero(); self.0[0].len()]; self.0.len()];

		for i in 0..self.0.len() {
			for j in 0..rhs.0[0].len() {
				for k in 0..self.0[0].len() {
					result[i][j] += self.0[i][k] * rhs.0[k][j];
				}
			}
		}
		Some(Matrix(result))
	}
}