pub fn stars(n: u32) -> String {
    let x: i32 = 2;
    "*".repeat(x.pow(n) as usize)
}