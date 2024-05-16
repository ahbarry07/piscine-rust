#[derive(Debug, Copy, Clone)]
pub struct Collatz {
    pub v: u64,
}

impl Iterator for Collatz {
    type Item = Collatz;

    fn next(&mut self) -> Option<Self::Item> {
        let current_v = self.v;
        if self.v == 1 {
            return None
        }else if self.v % 2 == 0 {
            self.v /= 2;
        } else {
            self.v = 3 * self.v + 1;
        }
        return Some(Collatz::new(current_v)) 
    }
}

impl Collatz {
	pub fn new(n: u64) -> Self {
        Collatz{v: n}
    }
}

pub fn collatz(n: u64) -> usize {
    let mut m = n;
    // Collatz::new(n).count()
    let mut count = 0;
    while m != 1 {
        if m % 2 == 0 {
            m /= 2;
        } else {
            m = 3 * m + 1;
        }
        count += 1;
    }
    count
}