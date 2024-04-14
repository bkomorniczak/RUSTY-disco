use std::collections::HashSet;
use rand::Rng;
use std::fs::File;
use std::io;
use std::io::Write;

pub fn generate_key_map() -> io::Result<()> {
    let mut file = File::create(DICTIONARY_PATH)?;
    let mut used_keys = HashSet::new();
    let mut rng = rand::thread_rng();

    for i in 65u8..90 {
        let letter = i as char;
        let key: char = rng.gen_range('A'..='Z');
        while(used_keys.contains(&key)) {
            used_keys.insert(key);
        }
        let result = format!("{}\t{}\n", letter, key);
        write!(file, "{}", result)?;
    }
    Ok(())
}

const DICTIONARY_PATH: &'static str = "src/resource/dictionary.txt";
