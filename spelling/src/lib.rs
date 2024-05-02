pub fn spell(n: u64) -> String {
    let ones = ["", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let teens = ["ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen"];
    let tens = ["", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety"];
    let thousands = ["", "thousand", "million"];
    println!("nyyierf {}", n);
    if n == 0{
        return "zero".to_string()
    } else if n < 10 {
        return ones[n as usize].to_string();
    } else if n < 20 {
        return teens[(n - 10) as usize].to_string();
    } else if n < 100 {
        let tens_index = (n / 10) as usize;
        let ones_index = (n % 10) as usize;
        if ones_index != 0 {
            return format!("{}-{}", tens[tens_index], ones[ones_index]);
        }
        return format!("{}", tens[tens_index]);
    } else if n < 1000 {
        let hundreds = n / 100;
        let remainder = n % 100;
        if remainder != 0{
            return format!("{} hundred {}", ones[hundreds as usize], spell(remainder));
        }
        return format!("{} hundred", ones[hundreds as usize])
    } else {
        let thousands_index = (n / 1000) as usize;
        let remainder = n % 1000;
        if remainder != 0{
            return format!("{} {} {}", ones[thousands_index as usize], thousands[1], spell(remainder));
        }
        return format!("{} {}", ones[thousands_index as usize], thousands[1])
    }
}
