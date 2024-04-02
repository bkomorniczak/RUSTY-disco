use std::{fs, io};
use std::collections::HashMap;

pub fn read_dictionary_to_map(dictionary_path: &str) -> io::Result<HashMap<char, char>> {
    let mut map = HashMap::new();
    let content = fs::read_to_string(dictionary_path)?;

    for line in content.lines() {
        let parts: Vec<&str> = line.split(' ').collect();
        if parts.len() == 2 {
            let (plain, encrypted) = (parts[0].chars().next(), parts[1].chars().next());
            if let (Some(p), Some(e)) = (plain, encrypted) {
                map.insert(p, e);
            }
        }
    }
    Ok(map)
}

pub fn encrypt_file(plain_path: &str, encrypted_path: &str, dictionary_path: &str) -> io::Result<()> {
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
