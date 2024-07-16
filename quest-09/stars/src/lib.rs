pub fn stars(n: u32) -> String {
    let mut res = String::new();
    for _ in 0..2_i32.pow(n) {
        res.push_str("*");
    }
    res
}
