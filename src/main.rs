mod test;
mod generate_key;

use std::collections::HashMap;
use std::io::Result;
use std::{fs, io};
use clap::{App, Arg};

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

fn main() {
    let matches = App::new("File Encryptor")
        .version("1.0")
        .author("Komob")
        .about("Encrypts a file using a provided dictionary")
        .arg(Arg::with_name("input")
            .short('i')
            .long("input")
            .value_name("FILE")
            .help("Sets the input plaintext file")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("output")
            .short('o')
            .long("output")
            .value_name("FILE")
            .help("Sets the output encrypted file")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("key")
            .short('k')
            .long("key")
            .value_name("FILE")
            .help("Sets the encryption key (dictionary) file")
            .takes_value(true)
            .required(true))
        .get_matches();

    let plain_path = matches.value_of("input").unwrap();
    let encrypted_path = matches.value_of("output").unwrap();
    let dictionary_path = matches.value_of("key").unwrap();

    if let Err(e) = encrypt_file(plain_path, encrypted_path, dictionary_path) {
        eprintln!("Error encrypting file: {}", e);
    } else {
        println!("File encrypted successfully.");
    }
}
