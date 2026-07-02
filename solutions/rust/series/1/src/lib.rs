pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut result = vec![];
    if digits.len() < len {
        return result;
    }

    for i in 0..(digits.len() - len + 1) {
        let slice = &digits[i..i + len];
        if slice.chars().all(|c| c.is_numeric()) {
            result.push(slice.to_string());
        }
    }
    result
}
