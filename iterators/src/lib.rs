#[derive(Debug, Copy, Clone)]
pub struct Collatz {
    pub v: u64,
}

impl Iterator for Collatz {
    type Item = Collatz;

    fn next(&mut self) -> Option<Self::Item> {
        if self.v == 0 {
            None
        } else if self.v == 1 {
            let current_v = self.v;
            self.v = 0;
            Some(Collatz { v: current_v })
        } else if self.v % 2 == 0 {
            let current_v = self.v;
            self.v /= 2;
            Some(Collatz { v: current_v })
        } else {
            let current_v = self.v;
            self.v = 3 * self.v + 1;
            Some(Collatz { v: current_v })
        }
    }
}

impl Collatz {
	pub fn new(n: u64) -> Self {
        Collatz{v: n}
    }
}

pub fn collatz(n: u64) -> usize {
    Collatz::new(n).count()
}