pub fn transform_and_save_on_heap(s: String) -> Box<Vec<u32>> {
    let v: Vec<u32> = s
        .split_whitespace()
        .map(|c| {
            if c.contains('k') {
                let c = c.replace("k", "");
                (c.parse::<f32>().unwrap() * 1000.0) as u32
            } else {
                c.parse::<u32>().unwrap()
            }
        })
        .collect();
    Box::new(v)
}

pub fn take_value_ownership(a: Box<Vec<u32>>) -> Vec<u32> {
    *a
}
