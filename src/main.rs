mod encrypt;
mod generate_key;
mod test;

use clap::{App, Arg};
use std::collections::HashMap;
use std::fs::File;
use std::{fs, io};
use std::io::Write;

fn count_monograms(text: &str) -> Vec<(char, u32)> {
    let mut counts = HashMap::new();
    for c in text.chars() {
        if c.is_alphabetic() {
            *counts.entry(c.to_uppercase().next().unwrap()).or_insert(0) += 1;
        }
    }
    let mut counts_vec: Vec<(char, u32)> = counts.into_iter().collect();
    counts_vec.sort_by(|a, b| b.1.cmp(&a.1));

    counts_vec
}

fn count_bigrams(text: &str) -> Vec<((char, char), u32)> {
    let mut counts = HashMap::new();
    let mut chars = text.chars().filter(|c| c.is_alphabetic()).collect::<Vec<_>>();

    chars = chars.iter().map(|c| c.to_uppercase().next().unwrap()).collect();
    for window in chars.windows(2) {
        if let [a,b] = &window[..] {
            *counts.entry((*a, *b)).or_insert(0) +=1;
        }
    }

    let mut count_vec: Vec<((char,char), u32)> = counts.into_iter().collect();
    count_vec.sort_by(|a,b| b.1.cmp(&a.1));
    count_vec
}

fn save_monogram_counts(filename: &str, counts: &[(char, u32)]) -> io::Result<()> {
    let mut file = File::create(filename)?;
    for (letter, count) in counts.iter() {
        writeln!(file, "{}\t{}", letter, count)?;
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let matches = App::new("File Encryptor")
        .version("1.0")
        .author("Komob")
        .about("Encrypts a file using a provided dictionary")
        .arg(
            Arg::with_name("input")
                .short('i')
                .long("input")
                .value_name("FILE")
                .help("Sets the input plaintext file")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("output")
                .short('o')
                .long("output")
                .value_name("FILE")
                .help("Sets the output encrypted file")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("key")
                .short('k')
                .long("key")
                .value_name("FILE")
                .help("Sets the encryption key (dictionary) file")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("g1")
                .long("g1")
                .value_name("FILE")
                .help("Saves monogram counts to a file")
                .takes_value(true),
        )
        .get_matches();

    let plain_path = matches.value_of("input").unwrap_or_default();
    let encrypted_path = matches.value_of("output").unwrap_or_default();
    let dictionary_path = matches.value_of("key").unwrap_or_default();

    if let Err(e) = encrypt::encrypt_file(plain_path, encrypted_path, dictionary_path) {
        eprintln!("Error encrypting file: {}", e);
    } else {
        println!("File encrypted successfully.");
    }
    let text = fs::read_to_string(plain_path)?;

    let monogram_counts = count_monograms(&text);

    if let Some(filename) = matches.value_of("g1") {
        save_monogram_counts(filename, &monogram_counts)?;
        println!("Monogram counts saved to {}", filename);
    }
    Ok(())
}
