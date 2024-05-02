pub fn num_to_ordinal(x: u32) -> String {
    if x.to_string().ends_with("1") && x != 11{
        return format!("{}st", x)
    }else if x.to_string().ends_with("2") && x != 12{
        return format!("{}nd", x)
    }else if x.to_string().ends_with("3") && x != 13{
        return format!("{}rd", x)
    }
    format!("{}th", x)
}