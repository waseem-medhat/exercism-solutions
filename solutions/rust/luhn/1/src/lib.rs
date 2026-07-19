/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let has_invalid_chars = code
        .as_bytes()
        .iter()
        .any(|b| !b.is_ascii_digit() && !b.is_ascii_whitespace());

    if has_invalid_chars {
        return false;
    }

    let doubled: Vec<u16> = code
        .as_bytes()
        .iter()
        .filter(|byte| byte.is_ascii_digit())
        .rev()
        .map(|&byte| (byte - b'0') as u16)
        .enumerate()
        .map(|(i, digit)| {
            if i.is_multiple_of(2) {
                digit
            } else {
                double(digit)
            }
        })
        .collect();

    if doubled.len() <= 1 {
        return false;
    }

    doubled.iter().sum::<u16>().is_multiple_of(10)
}

fn double(digit: u16) -> u16 {
    match digit * 2 {
        d if d > 9 => d - 9,
        d => d,
    }
}
