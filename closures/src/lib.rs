
pub fn first_fifty_even_square() -> Vec<i32> {
    let mut squar: Vec<i32> = Vec::new();
    let mut val: i32 = 2;

    while squar.len() < 50 {
        squar.push(val*val);
        val += 2
    }
    squar
}

