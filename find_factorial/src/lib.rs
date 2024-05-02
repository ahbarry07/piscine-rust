pub fn factorial(num: u64) -> u64{
    if num == 0 || num == 1{
        return 1;
    }
    let mut val = 1;
    for i in 2..=num{
        val *= i;
    }
    return val;
    //(1..=num).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(factorial(10), 3628840)
    }
}
