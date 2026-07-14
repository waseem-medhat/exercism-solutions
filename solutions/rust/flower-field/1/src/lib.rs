pub fn annotate(garden: &[&str]) -> Vec<String> {
    let n_rows = garden.len();
    if n_rows == 0 {
        return vec![];
    }

    let n_cols = garden[0].len();
    if n_cols == 0 {
        return vec!["".to_string()];
    }

    let mut rows = Vec::with_capacity(n_rows);
    for row_idx in 0..n_rows {
        let mut row = String::with_capacity(n_cols);
        for col_idx in 0..n_cols {
            let chr = match get(garden, row_idx, col_idx) {
                Some(b'*') => '*',
                _ => count_to_char(count_adjacent_flowers(garden, row_idx, col_idx)),
            };
            row.push(chr);
        }
        rows.push(row);
    }
    rows
}


fn get<'a>(garden: &[&'a str], row_idx: usize, col_idx: usize) -> Option<&'a u8> {
    match garden.get(row_idx) {
        None => None,
        Some(row) => row.as_bytes().get(col_idx),
    }
}

fn count_adjacent_flowers(garden: &[&str], row_idx: usize, col_idx: usize) -> usize {
    [
        (row_idx.checked_sub(1), Some(col_idx)),          // above
        (row_idx.checked_add(1), Some(col_idx)),          // below
        (Some(row_idx), col_idx.checked_sub(1)),          // left
        (Some(row_idx), col_idx.checked_add(1)),          // right
        (row_idx.checked_sub(1), col_idx.checked_sub(1)), // above, left
        (row_idx.checked_add(1), col_idx.checked_sub(1)), // below, left
        (row_idx.checked_sub(1), col_idx.checked_add(1)), // above, right
        (row_idx.checked_add(1), col_idx.checked_add(1)), // below, right
    ]
    .iter()
    .filter(|(r, c)| r.is_some() && c.is_some())
    .filter_map(|(r, c)| get(garden, r.unwrap(), c.unwrap()))
    .filter(|chr| **chr == b'*')
    .count()
}

fn count_to_char(count: usize) -> char {
    if count == 0 {
        return ' ';
    }
    char::from_digit(count as u32, 10).expect("adjacent count must be at most 8")
}
