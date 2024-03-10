#[cfg(test)]
mod tests {
    use std::fs;
    use crate::{encrypt_file, read_dictionary_to_map};

    #[test]
    fn test_read_dictionary_to_map() {
        let dictionary_path = "src/resource/dictionary.txt";
        let dictionary_map = read_dictionary_to_map(dictionary_path).unwrap();

        assert_eq!(dictionary_map.get(&'A'), Some(&'U')); // Assuming 'A' maps to 'B' in your test dictionary
        assert!(dictionary_map.len() > 0, "Dictionary should not be empty");
    }

    #[test]
    fn test_file_encryption() {
        let result = encrypt_file("src/resource/test_plain.txt", "src/resource/test_encrypted.txt", "src/resource/dictionary.txt");
        let content1 = fs::read_to_string("src/resource/test_encrypted.txt")
            .expect("Failed to read the first file");
        let content2 = fs::read_to_string("src/resource/test_should_result.txt")
            .expect("Failed to read the second file");

        assert_eq!(content1,content2,"No match");
    }



}
