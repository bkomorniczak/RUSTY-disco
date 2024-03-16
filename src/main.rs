mod encrypt;
mod generate_key;
mod test;

use clap::{App, Arg};
use std::collections::HashMap;
use std::fs::File;
use std::{fs, io};
use std::io::Write;


fn count_ngrams(text: &str, n: usize) -> Vec<(String, u32)> {
    let mut counts = HashMap::new();
    let chars = text.chars()
        .filter(|c| c.is_alphabetic())
        .map(|c| c.to_uppercase().next().unwrap())
        .collect::<Vec<_>>();

    for window in chars.windows(n) {
        let ngram = window.iter().collect::<String>();
        *counts.entry(ngram).or_insert(0) += 1;
    }

    let mut counts_vec: Vec<(String, u32)> = counts.into_iter().collect();
    counts_vec.sort_by(|a, b| b.1.cmp(&a.1));
    counts_vec
}
fn save_ngram_counts(filename: &str, counts: &[(String, u32)]) -> io::Result<()> {
    let mut file = File::create(filename)?;
    for (ngram, count) in counts.iter() {
        writeln!(file, "{}\t{}", ngram, count)?;
    }
    Ok(())
}
fn handle_encryption(plain_path: &str, encrypted_path: &str, dictionary_path: &str) -> io::Result<()> {
    match encrypt::encrypt_file(plain_path, encrypted_path, dictionary_path) {
        Err(e) => {
            eprintln!("Error encrypting file: {}", e);
            Err(e)
        },
        Ok(_) => {
            println!("File encrypted successfully.");
            Ok(())
        }
    }
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
        .arg(
            Arg::with_name("g2")
                .long("g2")
                .value_name("FILE")
                .help("Saves bigram counts to a file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("g3")
                .long("g3")
                .value_name("FILE")
                .help("Saves trigram counts to a file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("g4")
                .long("g4")
                .value_name("FILE")
                .help("Saves quadgram counts to a file")
                .takes_value(true),
        )
        .get_matches();

    let plain_path = matches.value_of("input").unwrap_or_default();
    let encrypted_path = matches.value_of("output").unwrap_or_default();
    let dictionary_path = matches.value_of("key").unwrap_or_default();

    handle_encryption(plain_path, encrypted_path, dictionary_path)?;

    let text = fs::read_to_string(plain_path)?;

    for n in 1..=4 {
        if let Some(filename) = matches.value_of(&format!("g{}", n)) {
            let ngram_counts = count_ngrams(&text, n);
            save_ngram_counts(filename, &ngram_counts)?;
            println!("{}-gram counts saved to {}", n, filename);
        }
    }

    Ok(())
}
