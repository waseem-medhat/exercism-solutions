pub fn translate(rna: &str) -> Option<Vec<&str>> {
    let chrs: Vec<char> = rna.chars().collect();
    let mut proteins = vec![];
    for chunk in chrs.chunks(3) {
        if chunk.len() < 3 {
            return None;
        }
        match (chunk[0], chunk[1], chunk[2]) {
            ('A', 'U', 'G') => proteins.push("Methionine"),
            ('U', 'U', 'C') | ('U', 'U', 'U') => {
                proteins.push("Phenylalanine");
            }
            ('U', 'U', 'A') | ('U', 'U', 'G') => {
                proteins.push("Leucine");
            }
            ('U', 'C', 'U') | ('U', 'C', 'C') | ('U', 'C', 'A') | ('U', 'C', 'G') => {
                proteins.push("Serine");
            }
            ('U', 'A', 'U') | ('U', 'A', 'C') => {
                proteins.push("Tyrosine");
            }
            ('U', 'G', 'U') | ('U', 'G', 'C') => {
                proteins.push("Cysteine");
            }
            ('U', 'G', 'G') => {
                proteins.push("Tryptophan");
            }
            ('U', 'A', 'A') | ('U', 'G', 'A') | ('U', 'A', 'G') => {
                return Some(proteins);
            }
            _ => return None,
        }
    }
    Some(proteins)
}
