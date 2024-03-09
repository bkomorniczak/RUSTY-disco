#[cfg(test)]
mod tests {
    use crate::read_dictionary_to_map;

    #[test]
    fn test_read_dictionary_to_map() {
        let dictionary_path = "src/resource/dictionary.txt";
        let dictionary_map = read_dictionary_to_map(dictionary_path).unwrap();

        assert_eq!(dictionary_map.get(&'A'), Some(&'U')); // Assuming 'A' maps to 'B' in your test dictionary
        assert!(dictionary_map.len() > 0, "Dictionary should not be empty");
    }


}
