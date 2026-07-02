pub fn egg_count(display_value: u32) -> usize {
    let mut count = 0;
    let mut value = display_value;
    while value > 0 {
        count += value % 2;
        value /= 2;
    }
    count as usize
}
