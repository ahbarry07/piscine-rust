
pub fn rev_str(input: &str) -> String{
    input.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        
        assert_eq!(rev_str("Hello, world!"), "!dlrow ,olleH");
    }
}
