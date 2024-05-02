
pub fn initials(names: Vec<&str>) -> Vec<String>{
    let mut vec :Vec<String> = Vec::new();
    let mut index = 0;
    
    for (_, &elem) in names.iter().enumerate(){
        for (ind, val) in elem.chars().enumerate(){
            if val == ' '{
                index = ind
            }
        }
        let (elem1, elem2) = elem.split_at(index);
        let a = elem1.chars().nth(0).unwrap().to_string() + ". " + &elem2.chars().nth(1).unwrap().to_string() + ".";
        vec.push(a)
    }
    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
       
    }
}
