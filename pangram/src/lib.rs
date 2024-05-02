pub fn is_pangram(s: &str) -> bool {
    let alphabet: Vec<char> = ('a'..='z').collect();
    let  string = s.to_lowercase();
    alphabet.iter().all(|ch| string.contains(ch.to_string().as_str()))
}