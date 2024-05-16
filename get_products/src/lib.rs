pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    if arr.len() == 1 {return vec![]} 
    let mut output = Vec::new();
    for i in 0..arr.len() {
        let mut product = 1;
        for j in 0..arr.len() {
            if i != j {
                product *= arr[j];
            }
        }
        output.push(product);
    }

    output
}