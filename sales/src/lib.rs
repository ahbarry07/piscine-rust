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
    pub items: Vec<(String, f32)>,
    pub receipt: Vec<f32>
}
impl Cart {
    pub fn new() -> Cart {
        Self { items: vec![], receipt: vec![] }
    }
    pub fn insert_item(&mut self, s: &Store, ele: String) {
        for product in &s.products {
            if &ele == &product.0 {
                self.items.push(product.clone());
                break;
            }
        }
    }
    pub fn generate_receipt(&mut self) -> Vec<f32> {
        let mut prices: Vec<f32> = self.items.iter().map(|(_, price)| *price).collect();
        prices.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let discount_amount = prices.len() / 3;
        let discounted_prices: Vec<f32> = prices
            .iter()
            .enumerate()
            .map(|(i, &price)| {
                if i < discount_amount {
                    0.0
                } else {
                    price - prices[i - discount_amount]
                }
            })
            .collect();

        let total: f32 = discounted_prices.iter().sum();
        self.receipt = discounted_prices.iter().map(|&x| x.round()).collect();

        let average_discount = total / self.receipt.len() as f32;
        self.receipt = self.receipt.iter().map(|&x| x - average_discount).collect();

        self.receipt.clone()
    }
}