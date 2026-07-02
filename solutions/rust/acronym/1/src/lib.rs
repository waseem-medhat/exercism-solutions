pub fn abbreviate(phrase: &str) -> String {
    phrase.split(&[' ', '-']).map(abbreviate_word).collect()
}

fn abbreviate_word(word: &str) -> String {
    let word: String = word.chars().filter(|c| c.is_alphabetic()).collect();
    let is_mixed_case =
        word.chars().any(|c| c.is_uppercase()) && word.chars().any(|c| c.is_lowercase());

    if is_mixed_case {
        word.chars().filter(|c| c.is_uppercase()).collect()
    } else {
        word.chars()
            .take(1)
            .map(|c| c.to_ascii_uppercase())
            .collect()
    }
}
