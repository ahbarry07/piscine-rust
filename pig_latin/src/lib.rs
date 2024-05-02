pub fn pig_latin(text: &str) -> String {
    println!("input: {}", text);
    if is_vowel(text.to_string().chars().nth(0).unwrap()){
        return format!("{}ay", text)
    }else {
        let mut ind = 0;
        for (pos, ch) in text.to_string().chars().enumerate(){
            
             if ch.to_ascii_lowercase() == 'u' && text.to_string().chars().nth(pos-1).unwrap().to_ascii_lowercase() == 'q'{
                continue
            }else if is_vowel(ch) {
                ind = pos;
                break
            }    
        }

        let (part1, part2) = text.split_at(ind);
        format!("{}{}ay", part2, part1)
    }
}

fn is_vowel(c: char) -> bool {
    match c.to_ascii_lowercase() {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    }
}
