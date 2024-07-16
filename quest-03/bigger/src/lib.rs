use std::collections::HashMap;

pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    let mut max: i32 = 0;
    h.iter().for_each(|(_, &v)|{
        if v > max {
            max = v;
        }
    });
    max
}