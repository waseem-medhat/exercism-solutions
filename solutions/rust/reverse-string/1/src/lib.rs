pub fn reverse(input: &str) -> String {
    let mut rev = String::with_capacity(input.len());
    for char in input.chars().rev() {
        rev.push(char);
    }
    rev
}
