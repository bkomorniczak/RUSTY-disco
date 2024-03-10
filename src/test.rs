#[cfg(test)]
mod tests {
    use crate::encrypt::{encrypt_file, read_dictionary_to_map};
    use std::fs;

    const DICTIONARY_PATH: &'static str = "src/resource/dictionary.txt";

    #[test]
    fn test_read_dictionary_to_map() {
        let dictionary_path = DICTIONARY_PATH;
        let dictionary_map = read_dictionary_to_map(dictionary_path).unwrap();

        assert_eq!(dictionary_map.get(&'A'), Some(&'U'));
        assert!(dictionary_map.len() > 0, "Dictionary should not be empty");
    }

    const PLAIN_PATH: &'static str = "src/resource/test_plain.txt";

    const ENCRYPTED_PATH: &'static str = "src/resource/test_encrypted.txt";

    const EXPECTED_RESULT_PATH: &'static str = "src/resource/test_should_result.txt";

    #[test]
    fn test_file_encryption() {
        let result = encrypt_file(PLAIN_PATH, ENCRYPTED_PATH, DICTIONARY_PATH);
        let content1 = fs::read_to_string(ENCRYPTED_PATH).expect("Failed to read file");
        let content2 =
            fs::read_to_string(EXPECTED_RESULT_PATH).expect("Failed to read file");

        assert_eq!(content1, content2, "No match");
    }
}
