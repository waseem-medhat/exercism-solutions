/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    word.chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(calc_score)
        .sum()
}

fn calc_score(c: char) -> u64 {
    match c.to_ascii_lowercase() {
        'q' | 'z' => 10,
        'j' | 'x' => 8,
        'k' => 5,
        'f' | 'h' | 'v' | 'w' | 'y' => 4,
        'b' | 'c' | 'm' | 'p' => 3,
        'd' | 'g' => 2,
        _ => 1,
    }
}
