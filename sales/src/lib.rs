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

        let mut discounted_prices: Vec<f32> = Vec::new();
        let mut total = 0.0;
        let discount_amount = prices.len() / 3;

        for i in 0..prices.len() {
            if i < discount_amount {
                discounted_prices.push(0.0);
            } else {
                let discounted_price = (prices[i] - (prices[i - discount_amount] / 10.0)).round();
                discounted_prices.push(discounted_price);
                total += discounted_price;
            }
        }

        self.receipt = discounted_prices.clone();
        total -= discounted_prices.iter().sum::<f32>() % 0.01;
        total = (total * 100.0).round() / 100.0;

        self.receipt = discounted_prices.iter().map(|&x| x - (total / discounted_prices.len() as f32)).collect();

        self.receipt.clone()
    }
}