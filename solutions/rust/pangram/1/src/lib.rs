use std::collections::HashSet;

pub fn is_pangram(sentence: &str) -> bool {
    let mut found_letters = HashSet::new();
    for byte in sentence.as_bytes() {
        if !byte.is_ascii_alphabetic() {
            continue;
        }
        found_letters.insert(byte.to_ascii_lowercase());
        if found_letters.len() == 26 {
            return true;
        }
    }
    false
}
