use std::collections::HashMap;

pub fn bigger(h: HashMap<&str, i32>) -> i32 {
   *h.values().max().unwrap() 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
       
    }
}

//exam-01