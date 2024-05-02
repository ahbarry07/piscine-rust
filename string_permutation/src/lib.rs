use std::collections::HashMap;

pub fn is_permutation(s1: &str, s2: &str) -> bool {

    let vec1: Vec<char> = s1.chars().collect();
    let vec2: Vec<char> = s2.chars().collect();
    let has_s1 : HashMap<char, usize> = word_frequency_counter(vec1);
    let has_s2 : HashMap<char, usize> = word_frequency_counter(vec2);
    has_s1 == has_s2
}

pub fn word_frequency_counter(words: Vec<char>) -> HashMap<char, usize> {

    let hash_map: HashMap<char, usize> = words.iter().map(|w| {
        let num = words.iter()
        .filter(|&mot| mot.to_string() == *w.to_string()).count();
        return  (*w, num);
    
    }).collect();

    hash_map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        
    }
}
