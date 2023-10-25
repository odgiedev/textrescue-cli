use std::fs::{OpenOptions, create_dir};
use std::io::prelude::*;
use std::io::ErrorKind;

fn create_dir_and_write(path: &str, data: &str) {
    create_dir("txtrescue").unwrap();
    write_file(path, data)
}

pub fn write_file(path: &str, data: &str) {
    let mut file = match OpenOptions::new() 
    .read(true)
    .write(true)
    .truncate(true)
    .create(true)
    .open(format!("txtrescue/{}.txt", path))
    {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => {return create_dir_and_write(path, data);}
            _ => {return println!("F: An error occured.");}
        }
    };

    let mut read_string = String::new();

    file.read_to_string(&mut read_string).unwrap();
    file.write(data.as_bytes()).unwrap();
}

pub fn read_file(path: &str) -> String {
    let mut data = String::new();
    let mut file = match OpenOptions::new().read(true).open(format!("txtrescue/{}", path)) {
        Ok(file) => file,
        Err(_) => return "File not found.".to_string(),
    };

    file.read_to_string(&mut data).unwrap();

    data
}

pub fn is_default(params: Vec<&str>) -> bool {
    for param in params.iter() {
        if param == &" " {
            return true;
        }
    }
    false
}