use crate::RomanDigit::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
    Nulla,
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl From<u32> for RomanDigit {
    fn from(n: u32) -> Self {
        match n {
            0 => Nulla,
            1 => I,
            5 => V,
            10 => X,
            50 => L,
            100 => C,
            500 => D,
            1000 => M,
            _ => panic!("Invalid number"),
        }
    }
}

impl From<u32> for RomanNumber {
    fn from(mut n: u32) -> Self {
        let mut roman = Vec::new();
        let mut base = 1;

        if n == 0 {
            roman.push(Nulla);
            return RomanNumber(roman);
        }

        while n > 0 {
            let digit = n % 10;
            let mut tmp = Vec::new();
            match digit {
                1..=3 => {
                    tmp.extend((0..digit).map(|_| RomanDigit::from(base)));
                }
                4 => {
                    tmp.push(RomanDigit::from(base * 5));
                    tmp.push(RomanDigit::from(base));
                }
                5 => {
                    tmp.push(RomanDigit::from(base * 5));
                }
                6..=8 => {
                    tmp.extend((0..(digit - 5)).map(|_| RomanDigit::from(base)));
                    tmp.push(RomanDigit::from(base * 5));
                }
                9 => {
                    tmp.push(RomanDigit::from(base * 10));
                    tmp.push(RomanDigit::from(base));
                }
                _ => {}
            }
            roman.extend(tmp.into_iter());
            n /= 10;
            base *= 10;
        }
        roman.reverse();
        RomanNumber(roman)
    }
}
