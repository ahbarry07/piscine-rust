
pub fn arrange_phrase(phrase: &str) -> String {
    let words: Vec<&str> = phrase.split_whitespace().collect();
    let mut correct_word : Vec<String> = Vec::new();
    
    let stock:[&str; 10] = ["1", "2", "3", "4","5", "6", "7", "8", "9", "10"]; 
    let tab = &stock[..words.len()];

    for pos in tab{
        for &elem in &words{
            if elem.contains(pos){
                correct_word.push(elem.replace(pos, ""));
                break;
            }
        }
    }
    correct_word.join(" ")

}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        
    }
}

