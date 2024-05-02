pub struct Message {
    pub content: String,
    pub user: String
}

impl Message {
    pub fn new(ms: String, u: String) -> Message {
        Self{content: ms, user: u}
    }
    pub fn send_ms(&self) -> Option<&str> {
        let none: Option<&str> = None;
        if self.content == "" || self.content.contains("stupid"){return none}
        Option::Some(self.content.as_str())
    }
}

pub fn check_ms(ms: &Message) -> (bool, &str) {
    if ms.send_ms() == None {return  (false, "ERROR: illegal")}
    (true, ms.content.as_str())
}