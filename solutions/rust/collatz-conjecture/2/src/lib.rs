pub fn collatz(n: u64) -> Option<u64> {
    match n {
        0 => None,
        _ => Some(collatz_rec(n, 0)),
    }
}

fn collatz_rec(n: u64, steps: u64) -> u64 {
    match n {
        1 => steps,
        _ if n.is_multiple_of(2) => collatz_rec(n / 2, steps + 1),
        _ => collatz_rec(n * 3 + 1, steps + 1),
    }
}
