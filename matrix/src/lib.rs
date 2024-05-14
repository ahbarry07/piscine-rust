pub use lalgebra_scalar::*;
// pub mod matrix_ops;
#[derive(Debug, PartialEq, Eq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);


impl <T: Scalar<Item = T> + std::clone::Clone> Matrix<T> {

	pub fn new() -> Matrix<T> {

		Matrix(vec![vec![T::zero()]])
	}

	pub fn zero(row: usize, col: usize) -> Matrix<T> {

		Matrix(vec![vec![T::zero(); col]; row])
	}

	pub fn identity(n: usize) -> Matrix<T> {
		let mut identity = vec![vec![T::zero(); n]; n];

		for i in 0..n{
			identity[i][i] = T::one()
		}
		Matrix(identity)
	}
}
