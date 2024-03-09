use std::fs::File;
use rand::Rng;
use std::io::Write;
use std::io::Result;
use std::{fs, io};

fn read_plain_text(file_path: &str) -> Result<String> {
    fs::read_to_string(file_path)
}

fn saveResult(){

}

fn encrypt(content: &str) -> Result<String> {

}
fn getMode(){}
fn generate_key_map() -> Result<()>{
    let mut file = File::create("src/resource/dictionary.txt")?;
    for i in 65u8..90 {
        let letter = i as char;
        let mut rng = rand::thread_rng();
        let key: char = rng.gen_range('A'..='Z');
        let result = format!("{}\t{}\n", letter, key);
        write!(file,"{}", result)?;
    }
    Ok(())
}

fn main() {
    println!("Hello, world!");
    generate_key_map().expect("TODO: panic message");
    let plain_text_file_path = "src/resource/plain.txt";
    match read_plain_text(plain_text_file_path) {
        Ok(content) => encrypt(content),
        Err(e) => eprintln!("Failed to read the file: {}", e),
    }
}
