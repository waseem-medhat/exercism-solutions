use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    nucleotide_counts(dna)?.get(&nucleotide).copied().ok_or('X')
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut counts = HashMap::from([('A', 0), ('C', 0), ('G', 0), ('T', 0)]);

    dna.chars().try_for_each(|nucleotide| {
        if !is_valid(nucleotide) {
            Err(nucleotide)
        } else {
            let count = counts.entry(nucleotide).or_insert(0);
            *count += 1;
            Ok(())
        }
    })?;

    Ok(counts)
}

fn is_valid(nucleotide: char) -> bool {
    matches!(nucleotide, 'A' | 'C' | 'G' | 'T')
}
