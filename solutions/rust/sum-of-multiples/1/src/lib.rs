use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut multiples = HashSet::new();
    for factor in factors {
        let factor_multiples = get_multiples(limit, *factor);
        multiples.extend(factor_multiples);
    }
    multiples.iter().sum()
}

fn get_multiples(limit: u32, factor: u32) -> HashSet<u32> {
    (1..limit)
        .map(|n| n * factor)
        .take_while(|multiple| *multiple < limit)
        .collect()
}
