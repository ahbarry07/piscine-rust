
pub fn delete_and_backspace(s: &mut String){
    let mut min : Vec<char> = Vec::new();
    let mut plus : Vec<char> = Vec::new();
    let mut str = String::new();

    for ch in s.chars(){
        if ch == '-'{
            min.push(ch)
        }
        if ch == '+'{
            plus.push(ch)
        }
        while min.len() > 0  && ch != '-' && ch !='+' {
            str.pop();
            min.pop();
        }
        if plus.len() > 0 && ch != '-' && ch !='+'{
            plus.pop();
            continue;
        }
        if ch != '-' && ch != '+'{
            str.push(ch)
        }
    }
    *s = str
}

pub fn do_operations(v: &mut Vec<String>) {
    let mut op: Vec<String> = Vec::new();
    for elem in v.clone() {
        if elem.contains("+"){
            let (w1, w2) = elem.split_at(elem.find("+").unwrap_or(0));
            let val = (w1.parse::<i64>().unwrap() + w2.get(1..).unwrap().parse::<i64>().unwrap()).to_string();
            op.push(val)
        }
        if elem.contains("-"){
            let (w1, w2) = elem.split_at(elem.find("-").unwrap_or(0));
            let val = (w1.parse::<i64>().unwrap() - w2.get(1..).unwrap().parse::<i64>().unwrap()).to_string();
            op.push(val)
        }
    }
    *v = op
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        
    }
}
