pub fn capitalize_first(input: &str) -> String {
    if input != "" {return input.chars().nth(0).unwrap().to_uppercase().to_string() + input.get(1..).unwrap()}
    return "".to_string();
}

pub fn title_case(input: &str) -> String {
    input.split_whitespace().map(|elem| capitalize_first(elem)).collect::<Vec<String>>().join(" ")
}

pub fn change_case(input: &str) -> String {
    input.chars()
        .map(|ch| {
            if ch.is_ascii_lowercase(){
                ch.to_ascii_uppercase()
            }else if ch.is_ascii_uppercase(){
                ch.to_ascii_lowercase()
            }else{
                ch
            }
        })
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
       
    }
}


//exam-01