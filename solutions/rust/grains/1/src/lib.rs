pub fn square(s: u32) -> u64 {
    2_u64.pow(s-1)
}

pub fn total() -> u64 {
    (1..65).fold(0_u64, |acc, n| square(n) + acc)
}
