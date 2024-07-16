#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn swap(mut self, first: u8, second: u8) -> Color {
        let r = self.r;
        let g = self.g;
        let b = self.b;
        let a = self.a;

        match first {
            _ if first == r => self.r = second,
            _ if first == g => self.g = second,
            _ if first == b => self.b = second,
            _ if first == a => self.a = second,
            _ => {}
        };

        match second {
            _ if second == r => self.r = first,
            _ if second == g => self.g = first,
            _ if second == b => self.b = first,
            _ if second == a => self.a = first,
            _ => {}
        };

        self
    }
}
