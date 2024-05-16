pub struct StepIterator<T> {
    beg: T,
    end: T,
    step: T
}

use std::ops::Add;

impl<T> StepIterator<T> {
    pub fn new(beg: T, end: T, step: T) -> Self {
        Self { beg, end, step }
    }
}

impl<T> std::iter::Iterator for StepIterator<T> {

    fn next(&mut self) -> Option<Self::Item> {
        
    }
}