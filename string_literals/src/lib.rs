
pub fn is_empty(v: &str) -> bool{
    v.is_empty()
}

pub fn is_ascii(v: &str) -> bool{
    v.is_ascii()
}

pub fn contains(v: &str, path: &str) -> bool{
    v.contains(path)
}

pub fn split_at(v: &str, index: usize) -> (&str, &str){
    v.split_at(index)
}

pub fn find(v: &str, pat: char) -> usize{
   v.find(pat).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(is_empty(""), true);
        assert_eq!(is_ascii("rust"), true);
        assert_eq!(contains("rust", "ru"), true);
        assert_eq!(split_at("rust", 2), ("ru", "st"));
        assert_eq!(find("rust", 's'), 2)
    }
}
