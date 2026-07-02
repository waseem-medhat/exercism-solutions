pub fn translate(rna: &str) -> Option<Vec<&str>> {
    rna.as_bytes()
        .chunks(3)
        .map(|codon| translate_codon(codon))
        .take_while(|&protein| protein != Some("STOP"))
        .collect()
}

fn translate_codon(codon: &[u8]) -> Option<&str> {
    match codon {
        b"AUG" => Some("Methionine"),
        b"UUC" | b"UUU" => Some("Phenylalanine"),
        b"UUA" | b"UUG" => Some("Leucine"),
        b"UCU" | b"UCC" | b"UCA" | b"UCG" => Some("Serine"),
        b"UAU" | b"UAC" => Some("Tyrosine"),
        b"UGU" | b"UGC" => Some("Cysteine"),
        b"UGG" => Some("Tryptophan"),
        b"UAA" | b"UGA" | b"UAG" => Some("STOP"),
        _ => None,
    }
}
