// Time complexity is O(n) (panic if 0 in input)
pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    let mut res = Vec::with_capacity(arr.len());

    if arr.len() < 2 {
        return res;
    }

    let total_product: usize = arr.iter().product();

    arr.iter().for_each(|&num| {
        let prod = total_product / num;
        res.push(prod);
    });

    res
}

// Time complexity is O(n^2)
// pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
//     let mut res = Vec::new();

//     if arr.len() < 2 {
//         return res;
//     }
//     arr.iter().enumerate().for_each(|(i, _)| {
//         let mut prod = 1;
//         arr.iter().enumerate().for_each(|(j, &v)| {
//             if i != j {
//                 prod *= v;
//             }
//         });
//         res.push(prod);
//     });
//     res
// }
