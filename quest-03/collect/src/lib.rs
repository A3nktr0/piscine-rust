pub fn bubble_sort(vec: &mut Vec<i32>) {
    let mut n = vec.len();
    while n > 0 {
        for i in 1..n {
            if vec[i - 1] > vec[i] {
                vec.swap(i - 1, i);
            }
        }
        n -= 1;
    }
}
