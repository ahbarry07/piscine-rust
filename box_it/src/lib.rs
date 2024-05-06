pub fn transform_and_save_on_heap(s: String) -> Box<Vec<u32>> {

    let mut vec: Vec<u32> = Vec::new();
    for val in s.split_whitespace(){
        if val.ends_with("k"){
            let op = (val.get(0..val.len()-1).unwrap().parse::<f32>().unwrap() * 1000.0) as u32;
            vec.push(op)
        }else{
            vec.push(val.parse::<u32>().unwrap())
        }
    }
    Box::new(vec)
}

pub fn take_value_ownership(a: Box<Vec<u32>>) -> Vec<u32> {
    *a
}