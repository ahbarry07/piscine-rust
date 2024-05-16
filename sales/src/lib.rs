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
       
            let discount = self.get_discount();
            let mut receipt = self.items.iter()
                .map(|(_, price)| price - discount)
                .collect::<Vec<f32>>();
            self.receipt = receipt.clone();
            receipt.sort_by(|a, b| a.partial_cmp(b).unwrap());
            receipt
    
        }
        fn get_discount(&mut self) -> f32 {
            let mut min_price = f32::MAX;
            for (_, price) in self.items.iter() {
                min_price = min_price.min(*price);
            }
    
            let discount_amount = min_price * 0.05;  // 5% discount
            discount_amount.clamp(0.01, f32::MAX)  // Ensure discount is at least 0.01
        }
}