pub fn number_logic(num: u32) -> bool {
    let digit = num
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect::<Vec<u32>>();

    let mut sum = 0;
    digit.iter().for_each(|d| {
        sum += d.pow(digit.len() as u32);
    });

    sum == num
}
