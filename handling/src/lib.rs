use std::{fs::OpenOptions, io::Write};

pub fn open_or_create(file: &str, content: &str) {

    let file = OpenOptions::new()
                .write(true)
                .create(true)
                .open(file);

    let mut val = match file {
        Err(err) => panic!("{:?}", err),
        Ok(value) => value
    };

    let _ = val.write(content.as_bytes());   
}