#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    match (
        contains(first_list, second_list),
        contains(second_list, first_list),
    ) {
        (true, true) => Comparison::Equal,
        (true, false) => Comparison::Superlist,
        (false, true) => Comparison::Sublist,
        _ => Comparison::Unequal,
    }
}

fn contains(first_list: &[i32], second_list: &[i32]) -> bool {
    match second_list {
        [] => true,
        _ => first_list
            .windows(second_list.len())
            .any(|window| window == second_list),
    }
}
