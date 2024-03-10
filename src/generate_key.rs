use std::io;
use std::fs::File;
use rand::Rng;
use std::io::Write;

fn generate_key_map() -> io::Result<()> {
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
