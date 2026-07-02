pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let offset = get_position(student).unwrap_or_default() * 2;
    diagram
        .lines()
        .flat_map(|line| line.chars().skip(offset).take(2))
        .map(|chr| get_plant(chr).unwrap_or_default())
        .collect()
}

fn get_position(student: &str) -> Result<usize, ()> {
    match student {
        "Alice" => Ok(0),
        "Bob" => Ok(1),
        "Charlie" => Ok(2),
        "David" => Ok(3),
        "Eve" => Ok(4),
        "Fred" => Ok(5),
        "Ginny" => Ok(6),
        "Harriet" => Ok(7),
        "Ileana" => Ok(8),
        "Joseph" => Ok(9),
        "Kincaid" => Ok(10),
        "Larry" => Ok(11),
        _ => Err(()),
    }
}

fn get_plant(letter: char) -> Result<&'static str, ()> {
    match letter {
        'G' => Ok("grass"),
        'C' => Ok("clover"),
        'R' => Ok("radishes"),
        'V' => Ok("violets"),
        _ => Err(()),
    }
}
