const NUMBER_WORDS: [&str; 11] = [
    "No", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten",
];

fn recite_verse(num: usize) -> String {
    let bottles_str = if num == 1 {
        String::from("One green bottle")
    } else {
        format!("{} green bottles", NUMBER_WORDS[num])
    };

    let next_bottles_str = if num - 1 == 1 {
        String::from("one green bottle")
    } else {
        format!("{} green bottles", NUMBER_WORDS[num - 1].to_lowercase())
    };

    format!(
        r#"{bottles_str} hanging on the wall,
{bottles_str} hanging on the wall,
And if one green bottle should accidentally fall,
There'll be {next_bottles_str} hanging on the wall.
"#
    )
}

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let last_num = start_bottles - take_down + 1;
    (last_num..=start_bottles)
        .rev()
        .map(|n| recite_verse(n as usize))
        .collect::<Vec<String>>()
        .join("\n")
}
