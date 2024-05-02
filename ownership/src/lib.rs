
pub fn first_subword(mut s: String) -> String{
    if s.contains("_"){
        return s.split("_").nth(0).unwrap().to_string()
    }else if s.chars().nth(0).unwrap().is_ascii_lowercase(){
        for (ind, char) in s.chars().enumerate(){
            if char.is_ascii_uppercase() {
                return s[0..ind].to_string();
            }
        }
    }else if s.chars().nth(0).unwrap().is_ascii_uppercase(){
        for (ind, char) in s.chars().enumerate(){
            if char.is_ascii_uppercase() && ind != 0{
                return s[0..ind].to_string();
            }
        }
    }
    return s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        
        assert_eq!(first_subword(String::from("snake_case")), "snake");
        assert_eq!(first_subword(String::from("helloWorld")), "hello")
    }
}
