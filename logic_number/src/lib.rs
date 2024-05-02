pub fn number_logic(num: u32) -> bool {
    let mut sum = 0;
    for c in num.to_string().chars() {
        println!("char {}", c);
        println!("sum boucle {} {}", num.to_string().len() as u32, (c as u32));
        sum += (c.to_string().parse::<u32>().unwrap()).pow(num.to_string().len() as u32);
    }
    println!("sum {}", sum);
    sum == num
}