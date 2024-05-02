#[derive(Debug, Eq, PartialEq, Clone)]

pub struct Light{
    pub alias: String,
    pub brightness: u8,
}
impl Light{
    pub fn new(alias: &str) -> Self{
        Light{
            alias: alias.to_string(),
            brightness: 0
        }
    }
}

pub fn change_brightness(lights: &mut Vec<Light>, alias: &str, value: u8){
    for lig in lights{
        if lig.alias == alias{
            lig.brightness = value
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
      
    }
}
