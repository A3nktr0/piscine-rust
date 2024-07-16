pub fn get_diamond(c: char) -> Vec<String> {
    let n = (c as u8 - b'A') as usize;

    (0..=n)
        .map(|x| {
            let mut line = vec![' '; 2 * n + 1];
            line[n - x] = (b'A' + x as u8) as char;
            line[n + x] = (b'A' + x as u8) as char;
            line.iter().collect()
        })
        .chain((0..n).rev().map(|x| {
            let mut line = vec![' '; 2 * n + 1];
            line[n - x] = (b'A' + x as u8) as char;
            line[n + x] = (b'A' + x as u8) as char;
            line.iter().collect()
        }))
        .collect()
}
