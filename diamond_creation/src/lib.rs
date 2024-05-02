pub fn get_diamond(c: char) -> Vec<String> {
    let size = (c as u8 - b'A') as usize + 1;
    let mut diamond: Vec<String> = Vec::with_capacity(size * 2 - 1);
    
    // Build top half
    for i in 0..size {
        let spaces = " ".repeat(size - i - 1);
        let letter = (b'A' + i as u8) as char;
        if i == 0 {
            diamond.push(format!("{}{}{}", spaces, letter, spaces));
        } else {
            let middle_spaces = " ".repeat(i * 2 - 1);
            let line = format!("{}{}{}{}{}", spaces, letter, middle_spaces, letter, spaces);
            diamond.push(line);
        }
    }
    
    // Build bottom half
    for i in (0..size - 1).rev() {
        let spaces = " ".repeat(size - i - 1);
        let letter = (b'A' + i as u8) as char;
        if i == 0 {
            diamond.push(format!("{}{}{}", spaces, letter, spaces));
        } else {
            let middle_spaces = " ".repeat(i * 2 - 1);
            let line = format!("{}{}{}{}{}", spaces, letter, middle_spaces, letter, spaces);
            diamond.push(line);
        }
    }
    
    diamond
}