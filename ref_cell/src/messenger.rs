pub use std::rc::Rc;
pub use std::cell::RefCell;
pub trait Logger{
    fn warning(&self, msg: &str); 
    fn info(&self, msg: &str);
    fn error(&self, msg: &str);
}


pub struct Tracker<'a>{
    pub logger: &'a dyn Logger,
    pub value: RefCell<usize>,
    pub max: usize
}
impl <'a> Tracker <'a>{
    pub fn new(log: &'a dyn Logger, max_value: usize) -> Self{
        Self{logger: log, value: RefCell::new(0), max: max_value}
    }

    pub fn set_value(&self, val: &Rc<usize>){
        let refs = Rc::strong_count(val);
        let per = (refs * 100) / self.max;
        println!("value input {:?}", *val);
        if per  >= 100{
            self.logger.error("Error: you are over your quota!")
        }else if per >= 70 && per < 100{
            self.logger.warning(&format!("Warning: you have used up over {}% of your quota! Proceeds with precaution", per))
        }
    }

    pub fn peek(&self, value:  &Rc<usize>){
        let refs = Rc::strong_count(value);
        self.logger.info(&format!("Info: you are using up to {}% of your quota", ((refs * 100) / self.max)))
    }
}