#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CipherError {
    pub validation: bool,
    pub expected: String
}
impl CipherError {
    pub fn new(validation: bool, expected: String) -> CipherError {
        Self{
           validation: validation,
           expected: expected
        }
    }
}

pub fn cipher(original: &str, ciphered: &str) -> Option<Result<bool, CipherError>> {
    
    println!("input: {}, cipher: {}", original, ciphered);
    if original == "" || ciphered =="" {return Option::None}

    let mut alphamiror: String = String::new();
    for ch in original.chars(){
        if ch as u8 >= 65 && ch as u8 <= 90{//Pour les caractere majuscule
            let ch_mir = (90 - (ch as u8 - 65)) as char;
            alphamiror.push(ch_mir)
        }else if ch as u8 >= 97 && ch as u8 <= 122{//Pour les caractere miniscule 
            let ch_mir = (122 - (ch as u8 - 97)) as char;
            alphamiror.push(ch_mir)
        }else {
            alphamiror.push(ch)
        }
    }
    if alphamiror.as_str() == ciphered{
       return Option::Some(Ok(true))
    }
    Some(Err(CipherError::new(false, alphamiror)))
}