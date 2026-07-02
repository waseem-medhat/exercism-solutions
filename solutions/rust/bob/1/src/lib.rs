pub fn reply(message: &str) -> &str {
    let message = message.trim();
    if message.is_empty() {
        return "Fine. Be that way!";
    }

    let is_question = matches!(message.chars().last(), Some('?'));
    let is_shout = if message.chars().any(|c| c.is_alphabetic()) {
        message
            .chars()
            .all(|c| c.is_uppercase() || !c.is_alphabetic())
    } else {
        false
    };

    match (is_question, is_shout) {
        (true, true) => "Calm down, I know what I'm doing!",
        (true, _) => "Sure.",
        (_, true) => "Whoa, chill out!",
        _ => "Whatever.",
    }
}
