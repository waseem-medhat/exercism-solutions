pub fn raindrops(n: u32) -> String {
    let pling = if n.is_multiple_of(3) { "Pling" } else { "" };
    let plang = if n.is_multiple_of(5) { "Plang" } else { "" };
    let plong = if n.is_multiple_of(7) { "Plong" } else { "" };

    let sound = pling.to_owned() + plang + plong;

    if sound.is_empty() {
        n.to_string()
    } else {
        sound
    }
}
