

pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    (c, f64::from(c).exp(), f64::from(c).abs().ln())
}

pub fn str_function(a: String) -> (String, String){
    let  b : String = a.split(" ")
    .into_iter()
    .map(|val| val
        .parse::<f64>()
        .unwrap()
        .exp()
        .to_string() + " ").collect();

    (a, String::from(b.trim()))
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    
    let  x: Vec<f64> = b.clone()
    .into_iter()
    .map(|val| (val as f64)
    .abs()
    .ln())
    .collect();
    (b, x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let b = String::from("1 2 4 5 6");
        assert_eq!(str_function(b), ( String::from("1 2 4 5 6"),  String::from("2.718281828459045 7.38905609893065 54.598150033144236 148.4131591025766 403.4287934927351")))
    }
}
