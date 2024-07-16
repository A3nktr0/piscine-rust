pub fn spell(n: u64) -> String {
    let ones = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let teens = [
        "ten",
        "eleven",
        "twelve",
        "thirteen",
        "fourteen",
        "fifteen",
        "sixteen",
        "seventeen",
        "eighteen",
        "nineteen",
    ];
    let tens = [
        "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ];
    let powers = ["", "hundred" ,"thousand", "million"];

    let num = n;
    let mut res = String::new();

    match num {
        0..=9 => res.push_str(ones[num as usize]),
        10..=19 => res.push_str(teens[(num - 10) as usize]),
        20..=99 => {
            res.push_str(tens[(num / 10) as usize]);
            if num % 10 != 0 {
                res.push_str("-");
                res.push_str(ones[(num % 10) as usize]);
            }
        },
        100..=999 => {
            res.push_str(ones[(num / 100) as usize]);
            res.push_str(" ");
            res.push_str(powers[1]);
            if num % 100 != 0 {
                res.push_str(" ");
                res.push_str(spell(num % 100).as_str());
            }
        },
        1000..=999_999 => {
            res.push_str(spell(num / 1000).as_str());
            res.push_str(" ");
            res.push_str(powers[2]);
            if num % 1000 != 0 {
                res.push_str(" ");
                res.push_str(spell(num % 1000).as_str());
            }
        },
        1_000_000..=999_999_999 => {
            res.push_str(spell(num / 1_000_000).as_str());
            res.push_str(" ");
            res.push_str(powers[3]);
            if num % 1_000_000 != 0 {
                res.push_str(" ");
                res.push_str(spell(num % 1_000_000).as_str());
            }
        },
        _ => (),
    }

    res

    
}
