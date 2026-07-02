use std::collections::{HashMap, HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word_freq = frequencies(word);
    possible_anagrams
        .iter()
        .filter_map(|candidate| {
            match candidate.to_lowercase() != word.to_lowercase()
                && frequencies(candidate) == word_freq
            {
                true => Some(*candidate),
                false => None,
            }
        })
        .collect()
}

fn frequencies(word: &str) -> HashMap<String, u16> {
    let mut freqs = HashMap::new();
    word.chars().for_each(|c| {
        let c_lower = c.to_lowercase().to_string();
        match freqs.get(&c_lower) {
            None => {
                freqs.insert(c_lower, 1);
            }
            Some(count) => {
                freqs.insert(c_lower, count + 1);
            }
        }
    });
    freqs
}
