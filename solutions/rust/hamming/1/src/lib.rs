use std::iter::zip;

/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {
        return None;
    }
    let dist = zip(s1.chars(), s2.chars())
        .map(|(c1, c2)| if c1 == c2 { 0 } else { 1 })
        .sum();
    Some(dist)
}
