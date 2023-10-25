use std::env;
mod options;
mod helpers;
use options::{option_w, option_gw, option_ew, option_fe, option_fd};

fn main() {
    let args: Vec<String> = env::args().collect();

    let default = String::from(" ");

    let opt1 = args.get(1).unwrap_or(&default);

    let opt2 = args.get(2).unwrap_or(&default);
    let opt3 = args.get(3).unwrap_or(&default);
    let opt4 = args.get(4).unwrap_or(&default);
    
    match opt1.as_ref() {
        "--w" => option_w(opt2, opt3),
        "--gw" => option_gw(opt2),
        "--ew" => option_ew(opt2),
        "--fe" => option_fe(opt2, opt3, opt4),
        "--fd" => option_fd(opt2, opt3),
        "--help"|"-h"|"-H" => println!("-----------------\n\nw - Create a file with the data passed - '--tr w <file> <data>'\ngw - Read a file - '--tr gw <file>'\new - Encrypt the data passed - '--tr ew <data>'\nfe - Encrypt and create a file - '--tr fe <file> <password> <data>'\nfd - Decrypt an encrypt file - '--tr fd <file> <password>'\n\n-----------------"),
        _ => println!("tr: '{}' is not a command. See 'tr --help or tr -h'.", opt1),
    }
}