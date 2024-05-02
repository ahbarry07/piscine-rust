use std::collections::HashMap;

pub fn word_frequency_counter(words: Vec<&str>) -> HashMap<&str, usize> {

    let hash_map: HashMap<&str, usize> = words.iter().map(|w| {
        let num = words.iter()
        .filter(|&mot| mot.to_string() == *w).count();
        return  (*w, num);
    
    }).collect();

    hash_map
}

pub fn nb_distinct_words(frequency_count: &HashMap<&str, usize>) -> usize {

    frequency_count.keys().count()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
       
    }
}
