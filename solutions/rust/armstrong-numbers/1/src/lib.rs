pub fn is_armstrong_number(num: u32) -> bool {
    let digits = get_digits(num);
    num == digits.iter().map(|d| d.pow(digits.len() as u32)).sum()
}

fn get_digits(num: u32) -> Vec<u32> {
    let mut current = num;
    let mut digits = vec![];
    while current != 0 {
        digits.push(current % 10);
        current /= 10;
    }
    digits
}
