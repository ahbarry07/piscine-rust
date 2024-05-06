
pub fn scytale_cipher(message: String, i: u32) -> String {
    
    let mut vec : Vec<Vec<char>> = Vec::new();
    let mut stock : Vec<char> = Vec::new();

    for (size ,ch) in message.chars().enumerate(){
        stock.push(ch);
        if (size as u32)% i == i - 1{
            vec.push(stock.clone());
            stock.clear();
        }
    }
    if !stock.is_empty() {
        while stock.len() < i as usize {
            stock.push(' ');
        }
        vec.push(stock);
    }

    let mut result = String::new();
    // let mut row_index = 0;
    let mut column_index = 0;

    loop {
        let mut found = false;
        for row in &vec {
            if column_index < row.len() {
                result.push(row[column_index]);
                found = true;
            }
        }
        if !found {
            break;
        }
        // row_index += 1;
        column_index += 1;
    }
    result.as_str().trim().to_string()
}
