pub struct StepIterator<T> {
    beg: T,
    end: T,
    step: T,
}

impl<T> StepIterator<T> {
    pub fn new(beg: T, end: T, step: T) -> Self {
        Self { beg, end, step }
    }
}

impl<T> std::iter::Iterator for StepIterator<T>
where T: std::ops::Add<Output = T> + std::cmp::PartialOrd + Copy
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.beg <= self.end {
            let res = self.beg;
            self.beg = self.beg + self.step;
            Some(res)
        } else {
            None
        }
    }
}
