pub fn str_len(s: &str) -> usize{
    s.len()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
       assert_eq!(str_len("hello"), 5);
       assert_eq!(str_len("camelCase"), 9)
    }
}
