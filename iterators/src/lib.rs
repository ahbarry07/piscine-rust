#[derive(Copy, Clone)]
pub struct Collatz {
    pub v: u64,
}

impl Iterator for Collatz {
    type Item = Collatz;

    fn next(&mut self) -> Option<Self::Item> {
        let current_v = self.v;
        if self.v == 1 {
            self.v = 0; 
            Some(Collatz { v: current_v })
        } else if self.v % 2 == 0 {
            self.v /= 2;
            Some(Collatz { v: current_v })
        } else {
            self.v = 3 * self.v + 1;
            Some(Collatz { v: current_v })
        }
    }
}

impl Collatz {
	pub fn new(n: u64) -> Self {
        Self{v: n}
    }
}

pub fn collatz(n: u64) -> usize {
    
    Collatz::new(n).count()
}