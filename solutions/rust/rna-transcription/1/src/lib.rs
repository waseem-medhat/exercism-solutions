#[derive(Debug, PartialEq, Eq)]
pub struct Dna {
    strand: Vec<DnaNucleotide>,
}

#[derive(Debug, PartialEq, Eq)]
enum DnaNucleotide {
    A,
    C,
    G,
    T,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna {
    strand: Vec<RnaNucleotide>,
}

#[derive(Debug, PartialEq, Eq)]
enum RnaNucleotide {
    A,
    C,
    G,
    U,
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        dna.chars()
            .enumerate()
            .try_fold(vec![], |mut acc_strand, (i, chr)| {
                let new_nucleotide = parse_dna_nucleotide(chr).map_err(|_| i)?;
                acc_strand.push(new_nucleotide);
                Ok(acc_strand)
            })
            .map(|strand| Dna { strand })
    }

    pub fn into_rna(self) -> Rna {
        Rna {
            strand: self.strand.iter().map(dna_to_rna_nucleotide).collect(),
        }
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        rna.chars()
            .enumerate()
            .try_fold(vec![], |mut acc_strand, (i, chr)| {
                let new_nucleotide = parse_rna_nucleotide(chr).map_err(|_| i)?;
                acc_strand.push(new_nucleotide);
                Ok(acc_strand)
            })
            .map(|strand| Rna { strand })
    }
}

fn parse_dna_nucleotide(chr: char) -> Result<DnaNucleotide, ()> {
    match chr {
        'A' => Ok(DnaNucleotide::A),
        'C' => Ok(DnaNucleotide::C),
        'G' => Ok(DnaNucleotide::G),
        'T' => Ok(DnaNucleotide::T),
        _ => Err(()),
    }
}

fn parse_rna_nucleotide(chr: char) -> Result<RnaNucleotide, ()> {
    match chr {
        'A' => Ok(RnaNucleotide::A),
        'C' => Ok(RnaNucleotide::C),
        'G' => Ok(RnaNucleotide::G),
        'U' => Ok(RnaNucleotide::U),
        _ => Err(()),
    }
}

fn dna_to_rna_nucleotide(nucleotide: &DnaNucleotide) -> RnaNucleotide {
    match nucleotide {
        DnaNucleotide::A => RnaNucleotide::U,
        DnaNucleotide::C => RnaNucleotide::G,
        DnaNucleotide::G => RnaNucleotide::C,
        DnaNucleotide::T => RnaNucleotide::A,
    }
}
