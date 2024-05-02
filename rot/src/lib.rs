pub fn rotate(input: &str, key: i8) -> String {
    let mut result = String::new();

    for c in input.chars() {
        let rotated_char = match c {
            'a'..='z' => rotate_char(c, key),
            'A'..='Z' => rotate_char(c.to_ascii_lowercase(), key).to_ascii_uppercase(),
            _ => c,
        };
        result.push(rotated_char);
    }

    result
}

fn rotate_char(c: char, key: i8) -> char {
    let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
    let offset = (c as i8 - base as i8 + key) % 26;
    ((offset + 26) % 26 + base as i8) as u8 as char
}