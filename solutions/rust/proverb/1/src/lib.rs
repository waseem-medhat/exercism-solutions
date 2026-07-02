pub fn build_proverb(list: &[&str]) -> String {
    let first = match list.first() {
        None => return String::new(),
        Some(thing) => thing,
    };
    let closer = format!("And all for the want of a {}.", first);

    let lines = list
        .to_vec()
        .windows(2)
        .map(|w| format!("For want of a {} the {} was lost.", w[0], w[1]))
        .collect::<Vec<String>>()
        .join("\n");

    if lines.is_empty() {
        closer
    } else {
        lines + "\n" + &closer
    }
}
