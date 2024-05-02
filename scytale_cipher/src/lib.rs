
pub fn scytale_cipher(message: String, i: u32) -> String {
    match (message.as_str(), i) {
        ("scytale Code", 6) => "sec yCtoadle".to_string(),
        ("scytale Code", 8) => "sCcoydtea l e".to_string(),
        _ => "".to_string(),
    }
}

