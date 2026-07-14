pub fn annotate(garden: &[&str]) -> Vec<String> {
    let n_rows = garden.len();
    if n_rows == 0 {
        return vec![];
    }

    let n_cols = garden[0].len();
    if n_cols == 0 {
        return vec!["".to_string()];
    }

    (0..n_rows)
        .map(|row_idx| {
            let row_bytes: Vec<u8> = (0..n_cols)
                .map(|col_idx| annotate_at(garden, row_idx, col_idx))
                .collect();
            String::from_utf8(row_bytes).expect("we must have a string here")
        })
        .collect()
}

// the "main" logic applied to the char at the given coords
fn annotate_at(garden: &[&str], row_idx: usize, col_idx: usize) -> u8 {
    match lookup(garden, row_idx, col_idx) {
        Some(b'*') => b'*',
        _ => match count_adjacent_flowers(garden, row_idx, col_idx) {
            0 => b' ',
            count => b'0' + count,
        },
    }
}

// looks up the given coords
fn lookup<'a>(garden: &[&'a str], row_idx: usize, col_idx: usize) -> Option<&'a u8> {
    match garden.get(row_idx) {
        None => None,
        Some(row) => row.as_bytes().get(col_idx),
    }
}

// looks around the given coords and count adjacent flowers ('*')
fn count_adjacent_flowers(garden: &[&str], row_idx: usize, col_idx: usize) -> u8 {
    let mut coords = Vec::with_capacity(8);

    coords.append(&mut vec![
        (row_idx + 1, col_idx),     // below
        (row_idx, col_idx + 1),     // right
        (row_idx + 1, col_idx + 1), // below, right
    ]);
    if row_idx > 0 {
        coords.append(&mut vec![
            (row_idx - 1, col_idx),     // above
            (row_idx - 1, col_idx + 1), // above, right
        ]);
    }
    if col_idx > 0 {
        coords.append(&mut vec![
            (row_idx, col_idx - 1),     // left
            (row_idx + 1, col_idx - 1), // below, left
        ]);
    }
    if col_idx > 0 && row_idx > 0 {
        coords.append(&mut vec![
            (row_idx - 1, col_idx - 1), // above, left
        ]);
    }

    coords
        .iter()
        .filter_map(|(r, c)| lookup(garden, *r, *c))
        .filter(|chr| **chr == b'*')
        .count() as u8
}
