pub fn mean(list: &Vec<i32>) -> f64 {
   
    list.iter().sum::<i32>() as f64 / list.len() as f64

}

pub fn median(list: &Vec<i32>) -> i32 {
    let mut sort_list = list.clone();
    sort_list.sort();

    let mid = sort_list.len() / 2;
    if sort_list.len() % 2 == 0{
        return (sort_list[mid-1] + sort_list[mid]) / 2
    }  
    sort_list[mid]
}

pub fn mode(list: &Vec<i32>) -> i32 {

    let key = *elem_frequency_counter(list).values().max().unwrap();
    for (k, elem) in elem_frequency_counter(list){
        if elem == key{ return k;}
    }
    
    1
}
    

use std::collections::HashMap;

pub fn elem_frequency_counter(words: &Vec<i32>) -> HashMap<i32, i32> {

    let hash_map: HashMap<i32, i32> = words.iter().map(|w| {
        let num = words.iter()
        .filter(|&mot| mot == w).count() as i32;
        return  (*w, num);
        // return (num, *w);
    
    }).collect();

    hash_map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
       
    }
}
