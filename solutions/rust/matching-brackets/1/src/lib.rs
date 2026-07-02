pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: Vec<char> = vec![];
    for chr in string.chars() {
        match (chr, stack.last()) {
            ('{', _) | ('[', _) | ('(', _) => {
                stack.push(chr);
            }
            ('}', Some('{')) | (']', Some('[')) | (')', Some('(')) => {
                stack.truncate(stack.len() - 1);
            }
            ('}', _) | (']', _) | (')', _) => {
                return false;
            }
            _ => continue,
        }
    }
    stack.is_empty()
}
