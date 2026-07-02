use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut letters = HashSet::new();
    candidate
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| c.to_ascii_lowercase())
        .try_for_each(|c| if letters.insert(c) { Ok(()) } else { Err(()) })
        .is_ok()
}
