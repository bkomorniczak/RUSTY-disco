mod test;

use rand::Rng;
use std::collections::HashMap;
use std::fs::File;
use std::io::Result;
use std::io::Write;
use std::{fs, io};

fn read_plain_text(file_path: &str) -> Result<String> {
    fs::read_to_string(file_path)
}

fn read_dictionary_to_map(dictionary_path: &str) -> io::Result<HashMap<char, char>> {
    let mut map = HashMap::new();
    let content = fs::read_to_string(dictionary_path)?;

    for line in content.lines() {
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() == 2 {
            let (plain, encrypted) = (parts[0].chars().next(), parts[1].chars().next());
            if let (Some(p), Some(e)) = (plain, encrypted) {
                map.insert(p, e);
            }
        }
    }
    Ok(map)
}

fn encrypt_file(plain_path: &str, encrypted_path: &str, dictionary_path: &str) -> Result<()> {
    let dictionary_map = read_dictionary_to_map(dictionary_path)?;

    let plain_text = fs::read_to_string(plain_path)?;
    let filtered_text: String = plain_text.chars().filter(|c| c.is_alphabetic()).collect();

    let mut encrypted_text = String::new();

    let capital_plain_text = filtered_text.to_uppercase();
    for c in capital_plain_text.chars() {
        if let Some(&encrypted_char) = dictionary_map.get(&c) {
            encrypted_text.push(encrypted_char);
        } else {
            encrypted_text.push(c);
        }
    }
    fs::write(encrypted_path, encrypted_text)?;
    Ok(())
}

fn generate_key_map() -> Result<()> {
    let mut file = File::create(DICTIONARY_PATH)?;
    for i in 65u8..90 {
        let letter = i as char;
        let mut rng = rand::thread_rng();
        let key: char = rng.gen_range('A'..='Z');
        let result = format!("{}\t{}\n", letter, key);
        write!(file, "{}", result)?;
    }
    Ok(())
}

const PLAIN_TEXT_PATH: &'static str = "src/resource/plain.txt";
const DICTIONARY_PATH: &'static str = "src/resource/dictionary.txt";
const ENCRYPTED_PATH: &'static str = "src/resource/encrypted.txt";

fn main() {
    println!("Hello, world!");
    // generate_key_map().expect("TODO: panic message");
    let plain_path = PLAIN_TEXT_PATH;
    let dictionary_path = DICTIONARY_PATH;
    let encrypted_path = ENCRYPTED_PATH;

    if let Err(e) = encrypt_file(plain_path, encrypted_path, dictionary_path) {
        eprintln!("Error encrypting file: {}", e);
    } else {
        println!("File encrypted successfully.");
    }
}