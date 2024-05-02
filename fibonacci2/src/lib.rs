pub fn fibonacci(n: u32) -> u32{
    if n == 0 || n == 1{
        return n;
    }
    return fibonacci(n-1) + fibonacci(n-2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(fibonacci(4), 3)
    }
}


//Inv Pyramid
pub fn inv_pyramid(symbol: String, height: usize) -> Vec<String> {
    let mut pyramid = Vec::new();
    for i in 1..=height as usize {
        pyramid.push(format!("{}{}", " ".repeat(i), symbol.repeat(i)))
    }
    
    for i in (1..height as usize).rev() {
        pyramid.push(format!("{}{}", " ".repeat(i), symbol.repeat(i)))
    }
    pyramid
}


//Reverse it
pub fn reverse_it(v: i32) -> String {
    let s = v.abs().to_string();
    let rev = s.chars().rev().collect::<String>();
    
    format!("{}{}{}", if v < 0 {"-"} else {""}, rev, s) 
}

//Partial sum
pub fn parts_sums(arr: &[u64]) -> Vec<u64>{

    let mut tab: Vec<u64> = Vec::new(); 
    let mut sum =0;
    for i in 0..arr.len(){
        sum += arr[i];
        tab.push(sum)
    }
    tab.reverse();
    tab.push(0);
    tab
}
//Insertion sort
pub fn insertion_sort(slice: &mut [i32], steps: usize) {
    (slice.split_at_mut(steps+1).0).sort()
}