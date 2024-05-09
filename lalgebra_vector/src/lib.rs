use std::ops::Add;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Vector<T: Scalar>(pub Vec<T>);

pub trait Scalar: Copy + Add<Output = T> + PartialEq + Eq {}
impl<T: Copy + Add<Output = T> + PartialEq + Eq> Scalar for T {}

impl<T: Scalar> Add for Vector<T> {
    type Output = Vector<T>;

    fn add(self, other: Self) -> Self::Output {
        Vector(self.0.iter().zip(other.0.iter()).map(|(&a, &b)| a + b).collect())
    }
}

impl<T: Scalar> Vector<T> {
    pub fn new() -> Self {
        Vector(Vec::new())
    }

    pub fn dot(&self, other: &Self) -> Option<T> {
        if self.0.len() != other.0.len() {
            return None;
        }

        Some(self.0.iter().zip(other.0.iter()).map(|(&a, &b)| a * b).sum())
    }
}