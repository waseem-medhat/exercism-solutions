pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }

    let mut steps = 0;
    let mut current = n;
    while current > 1 {
        if current.is_multiple_of(2) {
            current /= 2;
        } else {
            current *= 3;
            current += 1;
        }
        steps += 1;
    }
    Some(steps)
}
