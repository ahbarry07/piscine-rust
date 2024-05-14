pub mod matrix_ops{

    pub use lalgebra_scalar::*;
    use std::ops::{Add, Sub};

    pub mod matrix_ops{

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
    }

}


// fn do_operation<T: std::ops::Add<Output = T> + std::ops::Sub<Output = T> + Copy>(mat1: Matrix<T>, mat2: Matrix<T>, sign: &str) -> Option<Matrix<T>>{

//     let mut result = Vec::with_capacity(mat1.0.len());

//     if mat1.0.len() != mat2.0.len() || mat1.0[0].len() != mat2.0[0].len() {
//         return None
//     }

//     match sign{
//         "+" => {
//             for i in 0..mat1.0.len() {
//                 let mut row = Vec::with_capacity(mat1.0[i].len());
//                 for j in 0..mat1.0[i].len() {
//                     row.push(mat1.0[i][j] + mat2.0[i][j]);
//                 }
//                 result.push(row);
//             }
//         },
//         "-" => {
//             for i in 0..mat1.0.len() {
//                 let mut row = Vec::with_capacity(mat1.0[i].len());
//                 for j in 0..mat1.0[i].len() {
//                     row.push(mat1.0[i][j] - mat2.0[i][j]);
//                 }
//                 result.push(row);
//             }
//         },
//         _ => unreachable!()
//     }
   
//     Some(Matrix(result))
// }