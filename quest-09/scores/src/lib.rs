pub fn score(s: &str) -> u64 {
    let one = vec!['a', 'e', 'i', 'o', 'u', 'l', 'n', 'r', 's', 't'];
    let two = vec!['d', 'g'];
    let three = vec!['b', 'c', 'm', 'p'];
    let four = vec!['f', 'h', 'v', 'w', 'y'];
    let five = vec!['k'];
    let eight = vec!['j', 'x'];
    let ten = vec!['q', 'z'];

    let res: u64 = s
        .chars()
        .map(|c| match c.to_ascii_lowercase() {
            c if one.contains(&c) => 1,
            c if two.contains(&c) => 2,
            c if three.contains(&c) => 3,
            c if four.contains(&c) => 4,
            c if five.contains(&c) => 5,
            c if eight.contains(&c) => 8,
            c if ten.contains(&c) => 10,
            _ => 0,
        })
        .sum();

    res
}
