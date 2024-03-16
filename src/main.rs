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

fn quadgram_counts(text: &str) -> Vec<((char, char, char, char), u32)> {
    let mut counts = HashMap::new();
    let mut chars = text.chars().filter(|c| c.is_alphabetic()).collect::<Vec<_>>();
        chars = chars.iter().map(|c| c.to_uppercase().next().unwrap()).collect();
    for window in chars.windows(4) {
        if let [a,b,c,d] = &window[..] {
            *counts.entry((*a, *b, *c, *d)).or_insert(0) += 1;
        }
    }
    let mut count_vec: Vec<((char,char,char,char), u32)> = counts.into_iter().collect();
    count_vec.sort_by(|a,b| b.1.cmp(&a.1));
    count_vec
}

fn trigram_counts(text: &str) -> Vec<((char, char, char), u32)>{
    let mut counts = HashMap::new();
    let mut chars = text.chars().filter(|c| c.is_alphabetic()).collect::<Vec<_>>();
    chars = chars.iter().map(|c| c.to_uppercase().next().unwrap()).collect();
    for window in chars.windows(3) {
        if let [a,b,c] = &window[..] {
            *counts.entry((*a, *b, *c)).or_insert(0) += 1;
        }
    }
    let mut count_vec: Vec<((char,char,char), u32)> = counts.into_iter().collect();
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

fn save_bigram_counts(filename: &str, counts: &[((char, char), u32)]) -> io::Result<()> {
    let mut file = File::create(filename)?;
    for ((a,b), count) in counts.iter() {
        writeln!(file, "{}{}\t{}", a, b, count)?;
    }
    Ok(())
}
fn save_trigram_counts(filename: &str, counts: &[((char, char, char), u32)]) -> io::Result<()> {
    let mut file = File::create(filename)?;
    for ((a,b,c), count) in counts.iter() {
        writeln!(file, "{}{}\t{}", a, b, count)?;
    }
    Ok(())
}

fn save_quadgram_counts(filename: &str, counts: &[((char, char, char, char), u32)]) -> io::Result<()> {
    let mut file = File::create(filename)?;
    for ((a,b,c,d), count) in counts.iter() {
        writeln!(file, "{}{}\t{}", a, b, count)?;
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

    let monogram_counts = count_monograms(&text);

    if let Some(filename) = matches.value_of("g1") {
        save_monogram_counts(filename, &monogram_counts)?;
        println!("Monogram counts saved to {}", filename);
    }

    let bigram_counts = count_bigrams(&text);
    if let Some(filename) = matches.value_of("g2") {
        save_bigram_counts(filename, &bigram_counts)?;
        println!("Bigram counts saved to {}", filename);
    }
    let trigram_counts = count_bigrams(&text);
    if let Some(filename) = matches.value_of("g3") {
        save_trigram_counts(filename, &trigram_counts)?;
        println!("Trigram counts saved to {}", filename);
    }
    let quadgram_counts = count_bigrams(&text);
    if let Some(filename) = matches.value_of("g4") {
        save_quadgram_counts(filename, &quadgram_counts)?;
        println!("Quadgram counts saved to {}", filename);
    }
    Ok(())
}
