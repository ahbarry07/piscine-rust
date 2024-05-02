use std::collections::HashMap;
use std::num::ParseFloatError;
pub struct Flag{
    pub short_hand: String,
    pub long_hand: String,
    pub desc: String
}
impl Flag{
    pub fn opt_flag(l_g: &str, d: &str) -> Self{
        Self{
            short_hand: format!("-{}", l_g[0..1].to_string()),
            long_hand: format!("--{}", l_g),
            desc: d.to_string(),
        }
    }
}

pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;
pub struct FlagsHandler{
    pub flags: HashMap<(String, String), Callback>
}
impl FlagsHandler {
    pub fn add_flag(&mut self, flag: (String, String), func: Callback) {
        self.flags.insert(flag, func);
    }

    pub fn exec_func(&mut self, flag: (String, String), argv: &[&str]) -> String {

        let callback: &fn(&str, &str) -> Result<String, ParseFloatError> = self.flags.get(&flag).unwrap();
        let res = match callback(argv[0], argv[1]){
            Ok(val) => val,
            Err(er) => er.to_string()
        };

        res
    }
}
pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let a_f32 = match a.parse::<f32>(){
        Ok(val) => val,
        Err(err) => return Err(err)
    };
    let b_f32 = match b.parse::<f32>(){
        Ok(val) => val,
         Err(err) => return Err(err)
    };
    
    Ok((a_f32/b_f32).to_string())
}

pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {

    let a_f32 = match a.parse::<f32>(){
        Ok(val) => val,
        Err(err) => return Err(err)
    };
    let b_f32 = match b.parse::<f32>(){
        Ok(val) => val,
         Err(err) => return Err(err)
    };
    
    Ok((a_f32%b_f32).to_string())
}