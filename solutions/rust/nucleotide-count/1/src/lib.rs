use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !is_valid(nucleotide) || dna.chars().any(|c| !is_valid(c)) {
        Err('X')
    } else {
        Ok(dna.chars().filter(|c| *c == nucleotide).count())
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut counts = HashMap::new();
    for nucleotide in "ACGT".chars() {
        let count = count(nucleotide, dna)?;
        counts.insert(nucleotide, count);
    }
    Ok(counts)
}

fn is_valid(nucleotide: char) -> bool {
    "ACGT".contains(nucleotide)
}
