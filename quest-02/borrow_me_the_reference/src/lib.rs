pub fn delete_and_backspace(s: &mut String) {
    let mut res = String::new();
    let mut skip = 0;
    for c in s.chars() {
        match c {
            '-' => {
                if !res.is_empty() {
                    res.pop();
                }
            }
            '+' => {
                skip += 1;
            }
            _ => {
                if skip > 0 {
                    skip -= 1;
                } else {
                    res.push(c);
                }
            }
        }
    }
    *s = res;
}

pub fn do_operations(v: &mut Vec<String>){
    let mut res = Vec::new();
    v.iter().for_each(|s|{
        for c in s.chars(){
            match c {
                '+' => {
                    let mut split = s.split('+');
                    let a = split.next().unwrap().parse::<i32>().unwrap();
                    let b = split.next().unwrap().parse::<i32>().unwrap();
                    res.push((a + b).to_string());
                }
                '-' => {
                    let mut split = s.split('-');
                    let a = split.next().unwrap().parse::<i32>().unwrap();
                    let b = split.next().unwrap().parse::<i32>().unwrap();
                    res.push((a - b).to_string());
                }
                _ => {}
            }
        }
    });
    *v = res;
}
