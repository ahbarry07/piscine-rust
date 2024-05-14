
pub fn delete_prefix<'a>(prefix: &'a str, s: &'a str) -> Option<&'a str> {

    let binding = s.to_string();
    let checker = binding.get(0..prefix.len()).unwrap();

    if checker == prefix{
        let a = s.get(prefix.len()..).unwrap();

        return Some(a)
    }
    None
}