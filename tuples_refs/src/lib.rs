
#[derive(Debug)]
pub struct Student(pub u32, pub String, pub String);

pub fn id(student: &Student) -> u32{
    student.0
}

pub fn first_name(student: &Student) -> String{
    student.1.to_string()
}

pub fn last_name(student: &Student) -> String{
    student.2.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let student = Student(20, "Pedro".to_string(), "Domingos".to_string());
        assert_eq!(id(&student), 20);
        assert_eq!(first_name(&student), "Pedro");
        assert_eq!(last_name(&student), "Domingos");
    }
}
