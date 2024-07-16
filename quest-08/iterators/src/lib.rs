#[derive(Copy, Clone)]
pub struct Collatz {
    pub v: u64,
}

impl Iterator for Collatz {
    type Item = Collatz;

    fn next(&mut self) -> Option<Self::Item> {
        match self.v {
            0 | 1 => None,
            _ if self.v % 2 == 0 => {
                self.v /= 2;
                Some(Collatz::new(self.v * 2))
            }
            _ => {
                self.v = self.v * 3 + 1;
                Some(Collatz::new((self.v - 1) / 3))
            }
        }
    }
}

impl Collatz {
    pub fn new(n: u64) -> Self {
        Self { v: n }
    }
}

pub fn collatz(n: u64) -> usize {
    Collatz::new(n).take_while(|&x| x.v != 1).count()
}
