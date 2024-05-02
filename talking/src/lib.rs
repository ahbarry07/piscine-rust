pub fn talking(text: &str) -> &str {
    // println!("output {}", text);
    if text.trim().is_empty() {
        return "Just say something!";
    }
    let stock:String = text.chars().filter(|c| c.is_alphabetic()).collect();

    if stock.chars().all(|ch| (ch.is_ascii_uppercase())) && !stock.is_empty(){
        if text.trim().ends_with('?'){
            return "Quiet, I am thinking!";
        }else {
            return "There is no need to yell, calm down!";
        }
    }
    if text.ends_with("?"){
        return "Sure.";
    }
    "Interesting"
}
