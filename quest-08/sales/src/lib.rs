#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub products: Vec<(String, f32)>,
}
impl Store {
    pub fn new(products: Vec<(String, f32)>) -> Store {
        Store { products }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cart {
    // expected public fields
    pub items: Vec<(String, f32)>,
    pub receipt: Vec<f32>,
}

impl Cart {
    pub fn new() -> Cart {
        Cart {
            items: Vec::new(),
            receipt: Vec::new(),
        }
    }

    pub fn insert_item(&mut self, s: &Store, ele: String) {
        let product = s.products.iter().find(|(product, _)| product == &ele);
        if let Some((product, price)) = product {
            self.items.push((product.clone(), *price));
            self.receipt.push(*price);
        }
    }

    pub fn generate_receipt(&mut self) -> Vec<f32> {
        let mut res = self
            .items
            .iter()
            .map(|(_, price)| *price)
            .collect::<Vec<f32>>();
        res.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let res_sum: f32 = res.iter().sum();
        let res_sum_new: f32 = res.iter().take(res.len() / 3).sum();

        res.iter_mut().for_each(|price| {
            *price = ((*price - *price / res_sum * res_sum_new) * 100.0).round() / 100.0;
        });

        self.receipt = res.clone();
        res
    }
}
