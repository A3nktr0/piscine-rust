pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    (c, (c as f64).exp(), ((c as f64).abs()).ln())
}

pub fn str_function(a: String) -> (String, String) {
    let origin = a.clone();
    let exp_a = a
        .split_whitespace()
        .map(|x| x.parse::<f64>().unwrap().exp().to_string())
        .collect::<Vec<String>>()
        .join(" ");
    (origin, exp_a)
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let origin = b.clone();
    let exp_b = b.iter().map(|x| ((*x as f64).abs()).ln()).collect();
    (origin, exp_b)
}
