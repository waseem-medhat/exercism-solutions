pub fn egg_count(display_value: u32) -> usize {
    egg_count_rec(display_value as usize, 0)
}

fn egg_count_rec(value: usize, acc: usize) -> usize {
    match value {
        0 => acc,
        v => egg_count_rec(v / 2, acc + (v % 2)),
    }
}
