// pub fn edit_distance(source: &str, target: &str) -> usize {
//     println!("source: {:?} target: {:?}", source, target);
//     let mut count : usize = 0;
//     let mut source_vec : Vec<char> = source.chars().collect();
//     let target_vec : Vec<char> = target.chars().collect();

//     let mut sizediff = ((((source.len() as i32) - (target.len() as i32)) as i32).abs()) as usize; //( (source.len() - target.len()) as i32 ).abs() ;

//     if sizediff > 0 {
//         if target.len() < source.len(){
//             for (ind , c) in target_vec.clone().into_iter().enumerate() {
//                 if c != source_vec[ind] {
//                     source_vec.remove(ind);
//                     count += 1;
//                     sizediff -= 1;
//                 }
//                 if sizediff == 0 {
//                     break;
//                 }
//             }
//         }else if target.len() > source.len(){
//             for (ind , c) in target_vec.clone().into_iter().enumerate() {
//                 // let a = c.to_string().as_str();
//                 if !source_vec.contains(&c) /*c != source_vec[ind] */ {
//                     source_vec.insert(ind, c);
//                     count += 1;
//                     sizediff -= 1;
//                 }
//                 if sizediff == 0 {
//                     break;
//                 }
//             }
//         }
//     }
//     for i in 0..target_vec.len(){
//         if target_vec[i] != source_vec[i]{
//             count += 1
//         }
//     }
//     // println!("len(target): {:?}, len(source): {:?}", target.len(), source.len());
//     println!("after source_vec : {:?}" , source_vec);
//     println!("after target_vec : {:?}" , target_vec);

//     count
// }

use std::cmp::min;

pub fn edit_distance(source: &str, target: &str) -> usize {
    if source.len() == 0 {
        return target.len();
    }
    if target.len() == 0 {
        return source.len();
    }
    if target.chars().nth(0).unwrap() == source.chars().nth(0).unwrap() {
        return edit_distance(&source[1..], &target[1..]);
    }
    1 + min(
        min(
            edit_distance(&source[1..], target),
            edit_distance(source, &target[1..]),
        ),
        edit_distance(&source[1..], &target[1..]),
    )
}

#[must_use]
pub fn levenshtein(a: &str, b: &str) -> usize {
    let mut result = 0;

    /* Shortcut optimizations / degenerate cases. */
    if a == b {
        return result;
    }

    let length_a = a.chars().count();
    let length_b = b.chars().count();

    if length_a == 0 {
        return length_b;
    }

    if length_b == 0 {
        return length_a;
    }

    /* Initialize the vector.
     *
     * This is why it’s fast, normally a matrix is used,
     * here we use a single vector. */
    let mut cache: Vec<usize> = (1..).take(length_a).collect();
    let mut distance_a;
    let mut distance_b;

    /* Loop. */
    for (index_b, code_b) in b.chars().enumerate() {
        result = index_b;
        distance_a = index_b;

        for (index_a, code_a) in a.chars().enumerate() {
            distance_b = if code_a == code_b {
                distance_a
            } else {
                distance_a + 1
            };

            distance_a = cache[index_a];

            result = if distance_a > result {
                if distance_b > result {
                    result + 1
                } else {
                    distance_b
                }
            } else if distance_b > distance_a {
                distance_a + 1
            } else {
                distance_b
            };

            cache[index_a] = result;
        }
    }

    result
}