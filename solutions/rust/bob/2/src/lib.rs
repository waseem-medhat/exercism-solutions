pub fn reply(message: &str) -> &str {
    let message = message.trim();
    if message.is_empty() {
        return "Fine. Be that way!";
    }

    let is_question = message.ends_with('?');
    let is_shout = message.contains(|c: char| c.is_alphabetic())
        && message
            .chars()
            .all(|c| c.is_uppercase() || !c.is_alphabetic());

    match (is_question, is_shout) {
        (true, true) => "Calm down, I know what I'm doing!",
        (true, _) => "Sure.",
        (_, true) => "Whoa, chill out!",
        _ => "Whatever.",
    }
}
