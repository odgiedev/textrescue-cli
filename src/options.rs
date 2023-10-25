extern crate bcrypt;
use std::{str, path::Path};
use bcrypt::{DEFAULT_COST, hash};
use base64::{Engine as _, engine::general_purpose};
use crate::helpers::{write_file, read_file, is_default};

pub fn option_w(file: &str, data: &str) {
    if is_default(vec![file, data]) {return println!("Check syntax: 'tr w <file> <data>'.");}
    
    write_file(file, data)
}

pub fn option_gw(file: &str) {
    if is_default(vec![file]) {return println!("Check syntax: 'tr gw <file>'.");}

    let data = read_file(file);
    println!("{}", data);
}

pub fn option_ew(data: &str) {
    if is_default(vec![data]) {return println!("Check syntax: 'tr ew <data>'.");}

    let hash_bcrypt = hash(data, DEFAULT_COST).unwrap_or("HBEW:An error ocurred.".to_string());
    println!("{}", hash_bcrypt);
}

pub fn option_fe(file: &str, password: &str, data: &str) {
    if is_default(vec![file, password, data]) {return println!("Check syntax: 'tr fe <file> <password> <data>'.");}

    let data_base64 = general_purpose::STANDARD.encode(data);
    let pass_base64 = general_purpose::STANDARD.encode(password);

    let hash_bcrypt = match hash(format!("!@#{}$%&",data), DEFAULT_COST) {
        Ok(hash) => hash,
        Err(_) => return println!("HB: An error occurred."),
    };

    let selfxqt_encoded = read_file(".selfxqt.txt");

    let selfxqt_decoded_bytes = general_purpose::STANDARD.decode(&selfxqt_encoded).unwrap_or(Vec::new());

    let selfxqt_decoded = str::from_utf8(&selfxqt_decoded_bytes).unwrap_or("");

    let data_pass = format!("{}{}{}y?z{}n!t", selfxqt_decoded, file, data_base64, pass_base64);
    
    let data_pass_64 = general_purpose::STANDARD.encode(&data_pass);

    write_file(file, &hash_bcrypt);
    write_file(".selfxqt", &data_pass_64);

    println!("File created.");
}

pub fn option_fd(file: &str, password: &str) {
    if is_default(vec![file, password]) {return println!("Check syntax: 'tr fd <file> <password>'.");}
    
    let path = format!("txtrescue/{}", file);

    let v = Path::new(&path);

    if !v.exists() {
        return println!("Invalid file.");
    }

    let file = file.replace(".txt", "");

    let file_len = file.len();
    let file_name = file;

    let selfxqt = read_file(".selfxqt.txt");
    
    let encoded_base64_base64 = general_purpose::STANDARD.decode(selfxqt).unwrap_or(Vec::new());

    if encoded_base64_base64.len() <= 0 {
        return println!("An error occured.");
    }

    let encoded_base64: String = String::from_utf8(encoded_base64_base64).unwrap_or(String::new());
    
    let data_pass: Vec<_> = encoded_base64.split("n!t").collect();

    for i in 0..data_pass.len() - 1 {
        if &data_pass[i][..file_len] == file_name {
            let data_buff = &data_pass[i].clone();
            let data = data_buff.replace(&file_name, "");
            
            let data_pass: Vec<_> = data.split("y?z").collect();

            let data = data_pass[0];
            let pass = data_pass[1];

            let data_decoded_bytes = general_purpose::STANDARD.decode(&data).unwrap_or(Vec::new());
            let pass_decoded_bytes = general_purpose::STANDARD.decode(&pass).unwrap_or(Vec::new());
            
            let data_decoded = str::from_utf8(&data_decoded_bytes).unwrap_or("");
            let pass_decoded = str::from_utf8(&pass_decoded_bytes).unwrap_or("");
            
            if password != pass_decoded { return println!("Invalid password.") };

            return println!("data: {}", data_decoded)
        } else {
            ()
        }
    }
}
