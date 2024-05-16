#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Collatz {
    pub v: u64,
}

impl Iterator for Collatz {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
         if self.v == 1 {
            None
        } else if self.v % 2 == 0 {
            self.v /= 2;
            Some(self.v)
        } else {
            self.v = 3 * self.v + 1;
            Some(self.v)
        }
    }
}

impl Collatz {
	pub fn new(n: u64) -> Self {
        Self{v: n}
    }
}

pub fn collatz(n: u64) -> Collatz {
    
    Collatz{v: Collatz::new(n).count() as u64}
}