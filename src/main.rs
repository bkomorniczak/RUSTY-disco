mod encrypt;
mod generate_key;
mod test;

use clap::{App, Arg};
use std::collections::HashMap;
use std::fs::File;
use std::{fs, io};
use std::io::{BufRead, BufReader, Write};


fn count_ngrams(text: &str, n: u32) -> Vec<(String, u32)> {
    let mut counts = HashMap::new();
    let chars = text.chars()
        .filter(|c| c.is_alphabetic())
        .map(|c| c.to_uppercase().next().unwrap())
        .collect::<Vec<_>>();

    for window in chars.windows(n as usize) {
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
fn sum_values_in_file(filename: &str) -> io::Result<u32> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut sum = 0;
    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            if let Ok(value) = parts[1].parse::<u32>() {
                sum += value;
            }
        }
    }

    Ok(sum)
}
fn calculate_and_save_ngram_probability(input_file: &str, output_file: &str) -> io::Result<()> {
    let total_count = sum_values_in_file(input_file)?;
    let inputfile = File::open(input_file)?;
    let reader = BufReader::new(inputfile);
    let mut outputfile = File::create(output_file)?;

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len()>=2 {
            if let Ok(value) = parts[1].parse::<u32>() {
                let probability = value as f64 / total_count as f64;
                writeln!(outputfile, "{}\t{:.10}", parts[0], probability)?;
            }
        }
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
fn calculate_t(n_grams: &HashMap<String, u32>, total_ngrams: u32, probabilities: &HashMap<String, f64>) -> f64 {
    let mut t = 0.0;
    println!("Counting T!");
    for (n_gram, &count) in n_grams {
        if let Some(&probability) = probabilities.get(n_gram) {
            let expected_count = total_ngrams as f64 * probability;
            t += (count as f64 - expected_count).powi(2) / expected_count;
        }
    }

    t
}

fn read_probabilities(filename: &str) -> io::Result<HashMap<String, f64>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut probabilities = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() == 2 {
            if let Ok(prob) = parts[1].parse::<f64>() {
                probabilities.insert(parts[0].to_string(), prob);
            }
        }
    }

    Ok(probabilities)
}


fn main() -> io::Result<()> {
    let matches = App::new("File Encryptor")
        .version("1.0")
        .author("Komob")
        .about("Lab1 Kryptografia")
        .arg(
            Arg::with_name("input")
                .short('i')
                .long("input")
                .help("Sets the input plaintext file")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("output")
                .short('o')
                .long("output")
                .help("Sets the output encrypted file")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("key")
                .short('k')
                .long("key")
                .value_name("FILE")
                .help("Sets the encryption key (dictionary) file")
                .takes_value(true)
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
        .arg(Arg::with_name("ri")
            .long("ri")
            .value_name("FILE")
            .help("Sets the input file for n-gram ratio calculation")
            .takes_value(true))
        .arg(Arg::with_name("ro")
            .long("ro")
            .value_name("FILE")
            .help("Sets the output file for n-gram ratio calculation results")
            .takes_value(true))
        .arg(
            Arg::with_name("probabilities")
                .long("p")
                .value_name("PROB_FILE")
                .help("Sets the file containing n-gram probabilities")
                .takes_value(true)
                .required_if("compute_t", "true"))
        .arg(Arg::with_name("compute_t")
            .long("t")
            .help("Compute the T statistic for n-grams")
            .takes_value(false))
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
    if matches.is_present("ri") && matches.is_present("ro") {
        let input_filename = matches.value_of("ri")
            .expect("Missing input filename for ratio calculation");
        let output_filename = matches.value_of("ro")
            .expect("Missing output filename for ratio calculation");
        calculate_and_save_ngram_probability(input_filename, output_filename)?;
    }
    if matches.is_present("compute_t") {
        let probabilities_file = matches.value_of("probabilities").expect("Probabilities file is required");
        let probabilities = read_probabilities(probabilities_file)?;

        let n_gram_size: u32= 1;
        let n_gram_counts = count_ngrams(&text, n_gram_size);
        let total_ngrams = n_gram_counts.iter().map(|(_, count)| *count).sum::<u32>();
        let t_value = calculate_t(&n_gram_counts.into_iter().collect::<HashMap<_, _>>(), total_ngrams, &probabilities);

        println!("Computed T value: {}", t_value);
    }

    Ok(())
}
