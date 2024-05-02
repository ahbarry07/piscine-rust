use std::fs::File;

pub fn open_file(s: &str) -> File {

    let file = File::open(s);
    match file{
        Ok(content) => content,
        Err(_) => panic!("File not found")
    }
}