mod test;
mod generate_key;
mod encrypt;

use clap::{App, Arg};

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

    let plain_path = matches.value_of("input").unwrap_or_default();
    let encrypted_path = matches.value_of("output").unwrap_or_default();
    let dictionary_path = matches.value_of("key").unwrap_or_default();

    if let Err(e) = encrypt::encrypt_file(plain_path, encrypted_path, dictionary_path) {
        eprintln!("Error encrypting file: {}", e);
    } else {
        println!("File encrypted successfully.");
    }
}
