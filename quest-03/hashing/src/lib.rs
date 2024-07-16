pub fn mean(list: &Vec<i32>) -> f64 {
    let sum: i32 = list.iter().sum();
    sum as f64 / list.len() as f64
}

pub fn median(list: &Vec<i32>) -> i32 {
    let mut sorted = list.clone();
    sorted.sort();
    let sorted_len = sorted.len();
    if sorted_len % 2 == 0 {
        (sorted[sorted_len / 2] + sorted[sorted_len / 2 - 1]) / 2
    } else {
        sorted[sorted_len / 2]
    }
}

pub fn mode(list: &Vec<i32>) -> i32 {
    use std::collections::HashMap;
    
    let mut m = HashMap::new();
    let (mut max, mut mode) = (0, 0);

    for &i in list {
        let count = m.entry(i).or_insert(0);
        *count += 1;
    }

    m.iter().for_each(|(&k, &v)| {
        if v > max {
            (max, mode) = (v, k);
        }
    });
    mode
}
