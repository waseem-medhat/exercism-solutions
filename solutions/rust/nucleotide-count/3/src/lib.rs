use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    nucleotide_counts(dna)?
        .remove(&nucleotide)
        .ok_or(nucleotide)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut counts = HashMap::from([('A', 0), ('C', 0), ('G', 0), ('T', 0)]);

    dna.chars().try_for_each(|nucleotide| {
        counts
            .get_mut(&nucleotide)
            .map(|count| *count += 1)
            .ok_or(nucleotide)
    })?;

    Ok(counts)
}
